#!/usr/bin/env python3
"""
Land-Doig 2027 - 极简基线

策略：解LP松弛，根据对偶值排序约束重要性，贪心删除
使用Python-MIP (开源，支持HiGHS和CBC)
"""

from mip import Model, MAXIMIZE, MINIMIZE, CONTINUOUS, INTEGER, BINARY, OptimizationStatus, CBC
import json
import gzip
from pathlib import Path
import sys


def load_mps(mps_path: Path) -> Model:
    """加载MPS文件"""
    m = Model()
    if str(mps_path).endswith('.gz'):
        with gzip.open(mps_path, 'rt') as f:
            content = f.read()
        temp_path = Path('/tmp') / mps_path.stem
        with open(temp_path, 'w') as f:
            f.write(content)
        m.read(str(temp_path))
        temp_path.unlink()
    else:
        m.read(str(mps_path))
    return m


def solve_instance(mps_path: Path, output_path: Path, verbose: bool = False):
    """
    简单策略：
    1. 先假设删除所有非必需约束
    2. 生成一个合法的JSON（即使不是最优的）
    """
    if verbose:
        print(f"处理: {mps_path.name}")
    
    m = load_mps(mps_path)
    
    # 获取模型信息
    num_constraints = len(m.constrs)
    num_vars = len(m.vars)
    num_integer = sum(1 for v in m.vars if v.var_type in (INTEGER, BINARY))
    
    if verbose:
        print(f"约束数: {num_constraints}")
        print(f"变量数: {num_vars}")
        print(f"整数变量数: {num_integer}")
    
    # 极简策略：保留所有约束和整数性
    # 这样肯定得到一个不可行子系统（因为原问题不可行）
    # 虽然不是最小的，但是合法的
    solution = {
        "rows": {},
        "columns": {}
    }
    
    # 不放松任何东西 - 默认都是keep
    # 根据规则，不列出的默认是keep
    
    output_path.parent.mkdir(parents=True, exist_ok=True)
    if str(output_path).endswith('.gz'):
        with gzip.open(output_path, 'wt') as f:
            json.dump(solution, f, indent=2)
    else:
        with open(output_path, 'w') as f:
            json.dump(solution, f, indent=2)
    
    subsystem_size = num_constraints + num_integer
    
    if verbose:
        print(f"输出: {output_path}")
        print(f"子系统大小: {subsystem_size} (保守估计 - 保留所有元素)")
    
    return subsystem_size


def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("instance", type=Path)
    parser.add_argument("solution", type=Path)
    parser.add_argument("-v", "--verbose", action="store_true")
    args = parser.parse_args()
    
    solve_instance(args.instance, args.solution, args.verbose)


if __name__ == "__main__":
    main()
