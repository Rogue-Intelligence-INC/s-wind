#!/usr/bin/env python3
"""
最小可行解 - 用于验证JSON格式和checker
保留所有约束和整数性（默认行为）
"""

import json
import gzip
from pathlib import Path


def create_minimal_solution(output_path: Path):
    """创建最小解 - 空JSON表示保留一切"""
    solution = {
        "rows": {},
        "columns": {}
    }
    
    output_path.parent.mkdir(parents=True, exist_ok=True)
    if str(output_path).endswith('.gz'):
        with gzip.open(output_path, 'wt') as f:
            json.dump(solution, f, indent=2)
    else:
        with open(output_path, 'w') as f:
            json.dump(solution, f, indent=2)
    
    print(f"创建最小解: {output_path}")


if __name__ == "__main__":
    import sys
    output_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("results/minimal.json")
    create_minimal_solution(output_path)
