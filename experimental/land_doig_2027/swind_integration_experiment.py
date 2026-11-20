#!/usr/bin/env python3
"""
S-wind集成实验

探索：S-wind能否为IIS查找提供候选策略？

实验设计：
1. 手工实现几种简单的约束删除策略（确定性基线）
2. 记录每种策略在开发集上的性能序列（子系统大小、时间等）
3. 将这些数值序列输入S-wind的Fate引擎
4. 查看S-wind是否能识别模式或推荐参数
5. 如果S-wind给出有意义的候选，在留出集上验证

诚实性约束：
- 如果序列不呈现可拟合模式 → 如实报告，不强行拟合
- 如果S-wind未产生有用建议 → 如实报告，不伪造
- 记录真实的S-wind调用、输入、输出
"""

import subprocess
import json
from pathlib import Path
from typing import List, Dict, Tuple
import sys


def call_swind_fate(sequence: List[float], description: str = "") -> Dict:
    """
    调用S-wind Fate引擎分析序列
    
    Args:
        sequence: 有理数序列 (将被转换为整数或分数)
        description: 序列描述
        
    Returns:
        S-wind输出（如果成功）
    """
    print(f"\n{'='*60}")
    print(f"S-wind Fate调用")
    print(f"描述: {description}")
    print(f"序列: {sequence}")
    print(f"{'='*60}")
    
    # S-wind CLI接受逗号分隔的数字
    # 将浮点数转换为有理数字符串
    sequence_str = ",".join(str(int(x)) if x == int(x) else f"{int(x*1000)}/1000" 
                           for x in sequence)
    
    swind_bin = Path("/workspace/target/release/swind")
    if not swind_bin.exists():
        print(f"警告: S-wind二进制不存在于 {swind_bin}")
        print("尝试编译...")
        result = subprocess.run(
            ["cargo", "build", "--release", "--bin", "swind"],
            cwd="/workspace",
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            print(f"编译失败: {result.stderr}")
            return {"error": "swind binary not available", "stderr": result.stderr}
    
    # 调用swind explore
    result = subprocess.run(
        [str(swind_bin), "explore", sequence_str],
        capture_output=True,
        text=True,
        timeout=30
    )
    
    print(f"\n返回码: {result.returncode}")
    print(f"标准输出:\n{result.stdout}")
    if result.stderr:
        print(f"标准错误:\n{result.stderr}")
    
    # 尝试解析输出
    output = {
        "returncode": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "discovered_pattern": None
    }
    
    # 简单解析：查找公式/模式
    lines = result.stdout.split('\n')
    for line in lines:
        if "discovered" in line.lower() or "formula" in line.lower() or "pattern" in line.lower():
            output["discovered_pattern"] = line.strip()
    
    return output


def experiment_strategy_sequences():
    """
    实验：测试不同删除策略的性能序列
    
    这里我们模拟几种策略的结果，然后让S-wind分析
    在真实实验中，这些数字应该来自实际运行不同策略的结果
    """
    
    # 模拟策略A：按约束索引顺序删除
    # 假设在3个dev实例上的子系统大小序列
    strategy_a_sizes = [52, 45, 38]  # stein15inf, bell3ainf, aim-50的假设结果
    
    # 模拟策略B：随机删除
    strategy_b_sizes = [50, 47, 40]
    
    # 模拟策略C：某种启发式
    strategy_c_sizes = [48, 43, 35]
    
    print("实验场景：")
    print("假设我们尝试了3种不同的约束删除策略")
    print("每种策略在3个dev实例上的子系统大小：")
    print(f"  策略A: {strategy_a_sizes}")
    print(f"  策略B: {strategy_b_sizes}")
    print(f"  策略C: {strategy_c_sizes}")
    print("\n问题：S-wind能否从这些序列中识别出模式？")
    print("     能否推荐下一个策略的参数？")
    
    # 调用S-wind分析策略A的结果序列
    result_a = call_swind_fate(strategy_a_sizes, "策略A的子系统大小序列")
    
    # 调用S-wind分析策略B的结果序列
    result_b = call_swind_fate(strategy_b_sizes, "策略B的子系统大小序列")
    
    # 调用S-wind分析策略C的结果序列
    result_c = call_swind_fate(strategy_c_sizes, "策略C的子系统大小序列")
    
    # 尝试分析改进趋势序列
    # 策略A→B→C的改进: 2, 2, 3
    improvement_sequence = [
        strategy_a_sizes[0] - strategy_b_sizes[0],
        strategy_b_sizes[0] - strategy_c_sizes[0],
    ]
    result_improvement = call_swind_fate(improvement_sequence, "策略间改进幅度序列")
    
    # 汇总结果
    summary = {
        "experiment": "swind_strategy_sequence_analysis",
        "input_sequences": {
            "strategy_a": strategy_a_sizes,
            "strategy_b": strategy_b_sizes,
            "strategy_c": strategy_c_sizes,
            "improvement": improvement_sequence
        },
        "swind_results": {
            "strategy_a": result_a,
            "strategy_b": result_b,
            "strategy_c": result_c,
            "improvement": result_improvement
        },
        "conclusions": []
    }
    
    # 分析S-wind是否给出了有用的模式
    for key, result in summary["swind_results"].items():
        if result.get("discovered_pattern"):
            summary["conclusions"].append(f"{key}: S-wind发现模式 - {result['discovered_pattern']}")
        elif result.get("returncode") == 0:
            summary["conclusions"].append(f"{key}: S-wind执行成功，但未发现显著模式")
        else:
            summary["conclusions"].append(f"{key}: S-wind执行失败或不适用")
    
    # 保存结果
    output_path = Path("results/swind_experiment_log.json")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w') as f:
        json.dump(summary, f, indent=2, ensure_ascii=False)
    
    print(f"\n{'='*60}")
    print(f"实验日志已保存: {output_path}")
    print(f"{'='*60}\n")
    
    # 打印结论
    print("实验结论:")
    for conclusion in summary["conclusions"]:
        print(f"  - {conclusion}")
    
    # 诚实评估
    print("\n诚实性评估:")
    print("  1. S-wind是否真正产生了策略候选？")
    if any("发现模式" in c for c in summary["conclusions"]):
        print("     是 - S-wind识别出了数值模式")
    else:
        print("     否 - S-wind未识别出可用于策略生成的模式")
    
    print("  2. 识别出的模式是否对IIS查找有意义？")
    print("     需要领域专家判断。")
    print("     序列公式拟合 ≠ 算法策略生成")
    
    print("  3. 能否将发现冻结为独立Python？")
    if any("发现模式" in c for c in summary["conclusions"]):
        print("     理论上可以，如果模式有具体公式")
        print("     但这只是数值预测，不是完整的IIS求解算法")
    else:
        print("     无模式可导出")
    
    return summary


def main():
    """主实验入口"""
    print("="*70)
    print("Land-Doig 2027 S-wind集成实验")
    print("="*70)
    print()
    print("目标：探索S-wind（序列公式拟合引擎）能否为IIS查找贡献候选策略")
    print()
    
    summary = experiment_strategy_sequences()
    
    print("\n" + "="*70)
    print("实验完成")
    print("="*70)
    
    return summary


if __name__ == "__main__":
    main()
