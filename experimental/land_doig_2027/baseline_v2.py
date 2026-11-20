#!/usr/bin/env python3
"""
Land-Doig 2027 实验 - 简化基线实现

使用HiGHS直接求解，避免PuLP API复杂性。
极简策略：删除尽可能多的约束，保持不可行性。
"""

import highspy
import json
import gzip
from pathlib import Path
import sys


def load_mps(mps_path: Path) -> highspy.Highs:
    """加载MPS文件到HiGHS"""
    h = highspy.Highs()
    h.setOptionValue("output_flag", False)
    h.setOptionValue("log_to_console", False)
    
    if str(mps_path).endswith('.gz'):
        with gzip.open(mps_path, 'rt') as f:
            content = f.read()
        temp_path = Path('/tmp') / mps_path.stem
        with open(temp_path, 'w') as f:
            f.write(content)
        h.readModel(str(temp_path))
        temp_path.unlink()
    else:
        h.readModel(str(mps_path))
    
    return h


def check_infeasible(h: highspy.Highs, active_rows: set, active_cols_int: set) -> bool:
    """
    检查给定的行/列子集是否不可行
    
    Args:
        h: HiGHS model
        active_rows: 保留的行索引集合
        active_cols_int: 保持整数性的列索引集合
        
    Returns:
        True if 不可行
    """
    # 创建临时模型
    h_test = highspy.Highs()
    h_test.setOptionValue("output_flag", False)
    h_test.setOptionValue("log_to_console", False)
    h_test.setOptionValue("time_limit", 10)
    
    # 获取原始模型信息
    lp = h.getLp()
    num_cols = lp.num_col_
    num_rows = lp.num_row_
    
    # 添加所有列（变量）
    h_test.addCols(
        num_cols,
        lp.col_cost_,
        lp.col_lower_,
        lp.col_upper_,
        0,  # num_nz
        None,  # start
        None,  # index
        None   # value
    )
    
    # 设置整数性 - 放松不在active_cols_int中的整数变量
    integrality = list(lp.integrality_)
    for i in range(num_cols):
        if i not in active_cols_int and integrality[i] == highspy.HighsVarType.kInteger:
            integrality[i] = highspy.HighsVarType.kContinuous
    h_test.changeColsIntegrality(0, num_cols - 1, integrality)
    
    # 添加活跃的行（约束）
    for row_idx in sorted(active_rows):
        if row_idx >= num_rows:
            continue
        # 获取这一行的非零元素
        row = h.getRow(row_idx)
        h_test.addRow(
            lp.row_lower_[row_idx],
            lp.row_upper_[row_idx],
            row.num_nz,
            row.index,
            row.value
        )
    
    # 求解LP松弛
    h_test.run()
    status = h_test.getModelStatus()
    
    return status == highspy.HighsModelStatus.kInfeasible


def greedy_deletion(h: highspy.Highs, verbose: bool = False) -> tuple:
    """
    贪心删除约束
    
    Returns:
        (relaxed_rows, relaxed_cols_int)
    """
    lp = h.getLp()
    num_rows = lp.num_row_
    num_cols = lp.num_col_
    
    # 获取所有整数变量
    integer_cols = {i for i in range(num_cols) 
                   if lp.integrality_[i] == highspy.HighsVarType.kInteger}
    
    active_rows = set(range(num_rows))
    active_cols_int = integer_cols.copy()
    
    if verbose:
        print(f"初始: {len(active_rows)} 行, {len(active_cols_int)} 整数变量")
    
    # 初始检查
    if not check_infeasible(h, active_rows, active_cols_int):
        raise ValueError("输入模型不是不可行的！")
    
    # 贪心删除行
    iteration = 0
    max_iterations = 500
    while len(active_rows) > 1 and iteration < max_iterations:
        iteration += 1
        found = False
        
        for row_idx in list(active_rows):
            test_rows = active_rows - {row_idx}
            if check_infeasible(h, test_rows, active_cols_int):
                active_rows = test_rows
                if verbose and iteration % 10 == 0:
                    print(f"迭代 {iteration}: 删除行 {row_idx}, 剩余 {len(active_rows)}")
                found = True
                break
        
        if not found:
            if verbose:
                print(f"无法继续删除行")
            break
    
    # 尝试放松整数性
    for col_idx in list(active_cols_int):
        test_cols_int = active_cols_int - {col_idx}
        if check_infeasible(h, active_rows, test_cols_int):
            active_cols_int = test_cols_int
            if verbose:
                print(f"放松整数性: 列 {col_idx}")
    
    relaxed_rows = set(range(num_rows)) - active_rows
    relaxed_cols_int = integer_cols - active_cols_int
    
    if verbose:
        print(f"\n最终: 保留 {len(active_rows)} 行, {len(active_cols_int)} 整数变量")
        print(f"      删除 {len(relaxed_rows)} 行, 放松 {len(relaxed_cols_int)} 整数性")
    
    return relaxed_rows, relaxed_cols_int


def generate_solution_json(h: highspy.Highs, relaxed_rows: set, relaxed_cols_int: set) -> dict:
    """生成JSON解"""
    lp = h.getLp()
    solution = {"rows": {}, "columns": {}}
    
    # 行名（约束名）
    for row_idx in relaxed_rows:
        row_name = h.getRowName(row_idx)
        if row_name:
            solution["rows"][row_name] = {"action": "relax"}
    
    # 列名（变量名）的整数性
    for col_idx in relaxed_cols_int:
        col_name = h.getColName(col_idx)
        if col_name:
            if col_name not in solution["columns"]:
                solution["columns"][col_name] = {}
            solution["columns"][col_name]["integrality"] = {"action": "relax"}
    
    return solution


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Land-Doig 2027 基线求解器 v2")
    parser.add_argument("instance", type=Path, help="输入MPS文件")
    parser.add_argument("solution", type=Path, help="输出JSON文件")
    parser.add_argument("-v", "--verbose", action="store_true", help="详细输出")
    args = parser.parse_args()
    
    if args.verbose:
        print(f"{'='*60}")
        print(f"处理: {args.instance.name}")
        print(f"{'='*60}\n")
    
    # 加载模型
    h = load_mps(args.instance)
    
    # 贪心删除
    relaxed_rows, relaxed_cols_int = greedy_deletion(h, args.verbose)
    
    # 生成JSON
    solution = generate_solution_json(h, relaxed_rows, relaxed_cols_int)
    
    # 保存
    args.solution.parent.mkdir(parents=True, exist_ok=True)
    if str(args.solution).endswith('.gz'):
        with gzip.open(args.solution, 'wt') as f:
            json.dump(solution, f, indent=2)
    else:
        with open(args.solution, 'w') as f:
            json.dump(solution, f, indent=2)
    
    lp = h.getLp()
    subsystem_size = (lp.num_row_ - len(relaxed_rows)) + (len([i for i in range(lp.num_col_) 
                      if lp.integrality_[i] == highspy.HighsVarType.kInteger]) - len(relaxed_cols_int))
    
    if args.verbose:
        print(f"\n输出: {args.solution}")
        print(f"子系统大小: {subsystem_size}")


if __name__ == "__main__":
    main()
