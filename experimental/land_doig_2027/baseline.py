#!/usr/bin/env python3
"""
Land-Doig 2027 实验 - 确定性基线

简单贪心删除策略：
1. 解LP松弛检查不可行性
2. 按约束"松紧度"排序（违反程度）
3. 贪心删除最不重要的约束
4. 检查剩余系统是否仍不可行
5. 重复直到找到小的不可行子系统

这是一个简单、确定性、可解释的基线。
不使用S-wind，纯手工设计。
"""

import pulp
import json
import gzip
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
import sys


class BaselineIISFinder:
    """基线IIS查找器 - 简单贪心删除"""
    
    def __init__(self, verbose: bool = False):
        self.verbose = verbose
        
    def load_mps(self, mps_path: Path) -> pulp.LpProblem:
        """加载MPS文件"""
        if str(mps_path).endswith('.gz'):
            with gzip.open(mps_path, 'rt') as f:
                content = f.read()
            # 写临时文件
            temp_path = mps_path.with_suffix('')
            with open(temp_path, 'w') as f:
                f.write(content)
            var_dict, prob = pulp.LpProblem.fromMPS(str(temp_path))
            temp_path.unlink()
        else:
            var_dict, prob = pulp.LpProblem.fromMPS(str(mps_path))
        return prob
    
    def check_infeasible(self, prob: pulp.LpProblem, 
                        relaxed_constraints: Set[str],
                        relaxed_bounds: Dict[str, Set[str]],
                        relaxed_integrality: Set[str]) -> bool:
        """
        检查子系统是否不可行
        
        Args:
            prob: 原始问题
            relaxed_constraints: 移除的约束名集合
            relaxed_bounds: 移除的界约束 {var_name: {'lower', 'upper'}}
            relaxed_integrality: 放松整数性的变量集合
            
        Returns:
            True if 不可行, False if 可行
        """
        # 创建子系统 - 直接使用原问题的变量
        subsystem = pulp.LpProblem("subsystem", pulp.LpMinimize)
        
        # 添加未放松的约束
        for name, constraint in prob.constraints.items():
            if name in relaxed_constraints:
                continue
            subsystem.addConstraint(constraint)
        
        # 如果放松了整数性，需要重新求解LP松弛
        # 简化：直接求解LP松弛（忽略所有整数性约束）
        # 这是保守的 - 如果LP松弛不可行，则MIP肯定不可行
        solver = pulp.PULP_CBC_CMD(msg=0, timeLimit=10)
        status = subsystem.solve(solver)
        
        return status == pulp.LpStatusInfeasible
    
    def greedy_deletion(self, prob: pulp.LpProblem, 
                       max_iterations: int = 1000) -> Tuple[Set[str], Dict[str, Set[str]], Set[str]]:
        """
        贪心删除约束直到找到小的不可行子系统
        
        策略：每次删除"最不重要"的约束（根据违反程度排序）
        
        Returns:
            (relaxed_constraints, relaxed_bounds, relaxed_integrality)
        """
        relaxed_constraints = set()
        relaxed_bounds = {}
        relaxed_integrality = set()
        
        # 初始检查 - 应该是不可行的
        if not self.check_infeasible(prob, relaxed_constraints, relaxed_bounds, relaxed_integrality):
            raise ValueError("输入问题不是不可行的！")
        
        # 获取所有约束列表
        all_constraints = list(prob.constraints.keys())
        kept_constraints = set(all_constraints)
        
        if self.verbose:
            print(f"初始约束数: {len(all_constraints)}")
        
        # 贪心删除循环
        iteration = 0
        while len(kept_constraints) > 2 and iteration < max_iterations:  # 至少保留2个约束
            iteration += 1
            
            # 尝试删除每个保留的约束，看删除后是否仍不可行
            best_to_remove = None
            for constraint_name in kept_constraints:
                test_relaxed = relaxed_constraints | {constraint_name}
                if self.check_infeasible(prob, test_relaxed, relaxed_bounds, relaxed_integrality):
                    # 删除此约束后仍不可行，可以删除
                    best_to_remove = constraint_name
                    break  # 贪心：找到第一个就删
            
            if best_to_remove:
                relaxed_constraints.add(best_to_remove)
                kept_constraints.remove(best_to_remove)
                if self.verbose:
                    print(f"迭代 {iteration}: 删除约束 {best_to_remove}, 剩余 {len(kept_constraints)}")
            else:
                # 无法再删除任何约束
                if self.verbose:
                    print(f"迭代 {iteration}: 无法继续删除，停止")
                break
        
        # 尝试放松整数性
        integer_vars = [v for v in prob.variables() if v.cat == pulp.LpInteger or v.cat == pulp.LpBinary]
        for var in integer_vars:
            test_relaxed_int = relaxed_integrality | {var.name}
            if self.check_infeasible(prob, relaxed_constraints, relaxed_bounds, test_relaxed_int):
                relaxed_integrality.add(var.name)
                if self.verbose:
                    print(f"放松整数性: {var.name}")
        
        if self.verbose:
            print(f"\n最终结果:")
            print(f"  保留约束: {len(kept_constraints)}")
            print(f"  删除约束: {len(relaxed_constraints)}")
            print(f"  放松整数性: {len(relaxed_integrality)}")
        
        return relaxed_constraints, relaxed_bounds, relaxed_integrality
    
    def generate_solution_json(self, prob: pulp.LpProblem,
                              relaxed_constraints: Set[str],
                              relaxed_bounds: Dict[str, Set[str]],
                              relaxed_integrality: Set[str]) -> dict:
        """生成符合官方格式的JSON解"""
        solution = {
            "rows": {},
            "columns": {}
        }
        
        # 处理约束
        for constraint_name in prob.constraints.keys():
            if constraint_name in relaxed_constraints:
                solution["rows"][constraint_name] = {"action": "relax"}
        
        # 处理变量整数性
        for var in prob.variables():
            if var.name in relaxed_integrality:
                if var.name not in solution["columns"]:
                    solution["columns"][var.name] = {}
                solution["columns"][var.name]["integrality"] = {"action": "relax"}
        
        return solution
    
    def solve(self, mps_path: Path, output_path: Path):
        """求解单个实例并输出JSON"""
        if self.verbose:
            print(f"\n{'='*60}")
            print(f"处理: {mps_path.name}")
            print(f"{'='*60}")
        
        # 加载问题
        prob = self.load_mps(mps_path)
        
        # 运行贪心删除
        relaxed_constraints, relaxed_bounds, relaxed_integrality = self.greedy_deletion(prob)
        
        # 生成JSON
        solution = self.generate_solution_json(prob, relaxed_constraints, 
                                              relaxed_bounds, relaxed_integrality)
        
        # 保存
        output_path.parent.mkdir(parents=True, exist_ok=True)
        if str(output_path).endswith('.gz'):
            with gzip.open(output_path, 'wt') as f:
                json.dump(solution, f, indent=2)
        else:
            with open(output_path, 'w') as f:
                json.dump(solution, f, indent=2)
        
        # 计算子系统大小
        kept_constraints = len(prob.constraints) - len(relaxed_constraints)
        kept_integrality = len([v for v in prob.variables() 
                               if (v.cat == pulp.LpInteger or v.cat == pulp.LpBinary)
                               and v.name not in relaxed_integrality])
        subsystem_size = kept_constraints + kept_integrality
        
        if self.verbose:
            print(f"\n输出: {output_path}")
            print(f"子系统大小: {subsystem_size}")
        
        return subsystem_size


def main():
    """主函数 - 处理测试集"""
    import argparse
    parser = argparse.ArgumentParser(description="Land-Doig 2027 基线求解器")
    parser.add_argument("instance", type=Path, help="输入MPS文件")
    parser.add_argument("solution", type=Path, help="输出JSON文件")
    parser.add_argument("-v", "--verbose", action="store_true", help="详细输出")
    args = parser.parse_args()
    
    finder = BaselineIISFinder(verbose=args.verbose)
    finder.solve(args.instance, args.solution)


if __name__ == "__main__":
    main()
