# S-wind 工具集

## 概述

S-wind 是一个精确代数模拟系统，提供穷尽搜索优化、自动论文生成和3D模型创建功能。

## 🛠️ 可用工具

### 1. `swind` - 统一命令行工具

```bash
# 查看帮助
cargo run --release --bin swind help

# 列出所有项目
cargo run --release --bin swind list

# 运行单个项目优化
cargo run --release --bin swind optimize --project A1

# 生成论文
cargo run --release --bin swind paper --project A1 --format all
```

### 2. `batch_optimizer` - 批量优化器

自动执行所有商业化项目的穷尽搜索。

```bash
cargo run --release --bin batch_optimizer
```

**输出:**
- `batch_output/papers/` - 所有项目论文 (Markdown)
- `batch_output/models/` - 3D 模型 (OBJ)
- `batch_output/BATCH_SUMMARY.md` - 汇总报告

### 3. `paper_generator` - 论文生成器

生成 arXiv 标准格式的学术论文。

```bash
cargo run --release --bin paper_generator
```

**输出:**
- `papers_output/markdown/` - Markdown 论文
- `papers_output/latex/` - LaTeX 论文
- `papers_output/INDEX.md` - 论文索引

## 📋 项目清单

| 类别 | 项目数 | 代表项目 |
|------|--------|----------|
| 用户指定 | 3 | 建筑、电车、WiFi |
| 芯片/AI | 10 | LLM推理、微架构 |
| 电池/能源 | 8 | 快充、光伏储能 |
| 制造业 | 8 | 排产、良率 |
| 材料/化工 | 8 | 催化剂、涂层 |
| 生物医药 | 8 | 药物组合、mRNA |
| 航空/汽车 | 8 | eVTOL、电机 |
| 金融/商业 | 7 | 定价、风控 |
| 前沿创意 | 12 | 火星、深海、Jetpack |
| 医疗服务 | 4 | 手术排程、急救 |
| 智慧城市 | 6 | 交通信号、自动驾驶 |
| 网络/安全 | 6 | 后量子、5G/6G |
| 娱乐/媒体 | 5 | 游戏AI、视频编码 |
| 农业/食品 | 5 | 灌溉、垂直农场 |
| 体育/健康 | 4 | 训练负荷、配速 |
| 建筑/土木 | 5 | 超高层、桥梁 |
| 🌍 环保 | 8 | 碳中和、海洋清理 |
| 📈 市场问题 | 8 | 房价调控、医疗配置 |
| 🤝 人文/扶贫 | 10 | 精准扶贫、UBI |
| **总计** | **133** | |

## 🚀 快速开始

```bash
# 1. 进入目录
cd /home/jouly/AI/Valhalla/s-wind-sim

# 2. 编译所有工具
cargo build --release

# 3. 运行批量优化 (生成所有论文和3D模型)
cargo run --release --bin batch_optimizer

# 4. 生成 arXiv 格式论文
cargo run --release --bin paper_generator
```

## 📂 输出目录结构

```
s-wind-sim/
├── batch_output/               # 批量优化器输出
│   ├── papers/                 # 项目论文 (30篇)
│   ├── models/                 # 3D模型 (6个)
│   └── BATCH_SUMMARY.md        # 汇总报告
│
├── papers_output/              # 论文生成器输出
│   ├── markdown/               # Markdown 论文
│   ├── latex/                  # LaTeX 论文
│   └── INDEX.md                # 论文索引
│
└── swind_output/               # CLI 工具输出
    ├── *.json                  # 优化结果
    └── papers/                 # 单项目论文
```

## 📝 论文标准

所有生成的论文遵循 arXiv 标准：

- **结构**: Abstract, Introduction, Problem Formulation, Methodology, Results, Discussion, Conclusion
- **格式**: Markdown + LaTeX 双格式
- **分类**: 自动添加 arXiv categories (physics.flu-dyn, cs.CE, econ.GN, etc.)
- **可复现**: 完整参数空间、算法描述、数值验证

## 🔧 扩展

添加新项目:

1. 编辑 `src/bin/batch_optimizer.rs` 中的 `get_all_projects()`
2. 定义 `Project` 结构体
3. 添加评估函数（如需）
4. 添加 3D 模型生成函数（如需）

---

*S-wind v1.0 - 穷尽搜索 · 自动论文 · 3D模型*
