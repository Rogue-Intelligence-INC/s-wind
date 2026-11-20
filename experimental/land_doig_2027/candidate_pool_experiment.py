#!/usr/bin/env python3
"""
S-wind候选池实验 - 改进版

用户反馈："因为s-wind只给出最优解"

问题：之前的实验只测试了单次查询，每次只得到一个"最优"拟合。
改进：探索S-wind的候选池和多策略生成能力。

新实验设计：
1. 使用--save-pending将候选保存到知识库
2. 多次调用处理不同策略序列
3. 检查知识库中累积的候选池
4. 探索S-wind如何筛选和提升候选（support_count, confidence）
"""

import subprocess
import json
from pathlib import Path
from typing import List, Dict
import tempfile


class CandidatePoolExperiment:
    """探索S-wind候选池功能"""
    
    def __init__(self, kb_path: Path):
        self.kb_path = kb_path
        self.swind_bin = Path("/workspace/target/release/swind")
        self.call_log = []
        
    def call_swind_with_kb(self, sequence: List[float], description: str, 
                          mode: str = "verified") -> Dict:
        """
        调用S-wind并保存候选到知识库
        
        Args:
            sequence: 数值序列
            mode: 探索模式 (fast/verified/loop/full)
        """
        print(f"\n{'='*60}")
        print(f"S-wind调用 (模式: {mode})")
        print(f"描述: {description}")
        print(f"序列: {sequence}")
        print(f"{'='*60}")
        
        sequence_str = ",".join(str(int(x)) if x == int(x) else f"{int(x*1000)}/1000" 
                               for x in sequence)
        
        cmd = [
            str(self.swind_bin),
            "explore",
            sequence_str,
            "--mode", mode,
            "--kb", str(self.kb_path),
            "--save-pending"
        ]
        
        print(f"命令: {' '.join(cmd)}")
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        output = {
            "description": description,
            "sequence": sequence,
            "mode": mode,
            "returncode": result.returncode,
            "stdout": result.stdout,
            "stderr": result.stderr
        }
        
        self.call_log.append(output)
        
        print(f"\n返回码: {result.returncode}")
        print(f"输出:\n{result.stdout}")
        if result.stderr:
            print(f"错误:\n{result.stderr}")
        
        return output
    
    def load_kb(self) -> Dict:
        """加载知识库查看候选池"""
        if not self.kb_path.exists():
            return {"error": "KB file not found"}
        
        try:
            with open(self.kb_path, 'r') as f:
                return json.load(f)
        except Exception as e:
            return {"error": str(e)}
    
    def analyze_candidates(self, kb: Dict) -> Dict:
        """分析知识库中的候选"""
        analysis = {
            "total_candidates": 0,
            "by_confidence": {},
            "by_support_count": {},
            "by_discovery_type": {},
            "candidates": []
        }
        
        # 检查pending_review队列
        if "pending_review" in kb:
            candidates = kb["pending_review"]
            analysis["total_candidates"] = len(candidates)
            
            for candidate in candidates:
                # 按置信度分组
                conf = candidate.get("confidence", 0.0)
                conf_key = f"{conf:.1f}"
                analysis["by_confidence"][conf_key] = analysis["by_confidence"].get(conf_key, 0) + 1
                
                # 按支持计数分组
                support = candidate.get("support_count", 0)
                analysis["by_support_count"][support] = analysis["by_support_count"].get(support, 0) + 1
                
                # 按发现类型分组
                disc_type = candidate.get("discovery_type", "unknown")
                analysis["by_discovery_type"][disc_type] = analysis["by_discovery_type"].get(disc_type, 0) + 1
                
                # 收集候选信息
                analysis["candidates"].append({
                    "id": candidate.get("id"),
                    "description": candidate.get("description"),
                    "formula": candidate.get("formula"),
                    "confidence": conf,
                    "support_count": support,
                    "discovery_type": disc_type
                })
        
        return analysis
    
    def run_experiment(self):
        """运行完整实验"""
        print("="*70)
        print("S-wind候选池实验 - 多候选生成与筛选")
        print("="*70)
        print(f"\n知识库路径: {self.kb_path}")
        
        # 实验场景：不同IIS策略的性能序列
        scenarios = [
            {
                "name": "策略A - 按索引顺序删除",
                "sequence": [52, 45, 38],
                "mode": "verified"
            },
            {
                "name": "策略B - 随机删除",
                "sequence": [50, 47, 40],
                "mode": "verified"
            },
            {
                "name": "策略C - 按松紧度删除",
                "sequence": [48, 43, 35],
                "mode": "verified"
            },
            {
                "name": "策略D - 组合启发式",
                "sequence": [46, 40, 33],
                "mode": "verified"
            },
            {
                "name": "策略E - 自适应权重",
                "sequence": [45, 39, 32],
                "mode": "verified"
            }
        ]
        
        # 多次调用S-wind，累积候选到知识库
        print("\n阶段1: 多次调用S-wind，累积候选")
        print("-" * 60)
        
        for scenario in scenarios:
            self.call_swind_with_kb(
                scenario["sequence"],
                scenario["name"],
                scenario["mode"]
            )
        
        # 加载并分析知识库
        print("\n阶段2: 分析知识库中的候选池")
        print("-" * 60)
        
        kb = self.load_kb()
        if "error" in kb:
            print(f"错误: {kb['error']}")
            return {"error": kb["error"]}
        
        analysis = self.analyze_candidates(kb)
        
        print(f"\n候选池统计:")
        print(f"  总候选数: {analysis['total_candidates']}")
        print(f"\n按置信度分布:")
        for conf, count in sorted(analysis['by_confidence'].items()):
            print(f"    {conf}: {count} 个")
        print(f"\n按支持计数分布:")
        for support, count in sorted(analysis['by_support_count'].items()):
            print(f"    {support} 次支持: {count} 个")
        print(f"\n按发现类型分布:")
        for dtype, count in analysis['by_discovery_type'].items():
            print(f"    {dtype}: {count} 个")
        
        if analysis['candidates']:
            print(f"\n候选详情:")
            for i, cand in enumerate(analysis['candidates'][:5], 1):  # 显示前5个
                print(f"\n  候选 {i}:")
                print(f"    描述: {cand['description']}")
                print(f"    公式: {cand.get('formula', 'N/A')}")
                print(f"    置信度: {cand['confidence']}")
                print(f"    支持计数: {cand['support_count']}")
                print(f"    类型: {cand['discovery_type']}")
        
        # 保存结果
        result = {
            "experiment": "swind_candidate_pool",
            "kb_path": str(self.kb_path),
            "call_log": self.call_log,
            "kb_snapshot": kb,
            "analysis": analysis
        }
        
        output_path = Path("results/candidate_pool_experiment.json")
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w') as f:
            json.dump(result, f, indent=2, ensure_ascii=False)
        
        print(f"\n实验结果已保存: {output_path}")
        
        # 评估
        print("\n" + "="*70)
        print("实验评估")
        print("="*70)
        
        print("\n1. S-wind是否产生了多个候选？")
        if analysis['total_candidates'] > 1:
            print(f"   ✅ 是 - 共产生 {analysis['total_candidates']} 个候选")
        elif analysis['total_candidates'] == 1:
            print(f"   ⚠️ 只有1个候选")
        else:
            print(f"   ❌ 否 - 没有候选保存到知识库")
        
        print("\n2. 候选是否有置信度和支持计数？")
        if analysis['by_confidence'] and analysis['by_support_count']:
            print(f"   ✅ 是 - 候选带有质量评估")
        else:
            print(f"   ❌ 否 - 无质量评估信息")
        
        print("\n3. 这些候选能否用于策略筛选？")
        if analysis['total_candidates'] > 1:
            print(f"   ⚠️ 理论上可以，但候选是公式拟合，不是策略算法")
            print(f"   📊 可以比较不同策略序列的拟合质量")
            print(f"   ❌ 但仍然不能**生成**新的IIS求解策略")
        else:
            print(f"   ❌ 候选不足，无法筛选")
        
        return result


def main():
    """主函数"""
    # 创建临时知识库
    kb_path = Path("/workspace/experimental/land_doig_2027/land_doig_kb.json")
    
    exp = CandidatePoolExperiment(kb_path)
    result = exp.run_experiment()
    
    print("\n" + "="*70)
    print("实验完成")
    print("="*70)
    
    return result


if __name__ == "__main__":
    main()
