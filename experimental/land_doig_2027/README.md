# Land–Doig 2027 MIP Competition实验

**实验日期**: 2026-11-20  
**基准commit**: `c21cac6` (Fix KB data errors, measured Blondle counts, industry exhaustive-optimization proof)  
**分支**: `cursor/land-doig-experiment-17b2`

## 实验目标

探索S-wind（作为算法产生者）能否为2027 Land–Doig MIP Competition（找小不可行子系统）贡献候选策略，并冻结为独立Python实现。

## 竞赛背景

- **竞赛**: 2027 Land–Doig MIP Competition - "Explainability and Small Infeasible Subsystems"
- **官网**: https://www.mixedinteger.org/2027/competition/
- **官方仓库**: https://github.com/mixedinteger/MIPcc27
- **问题**: 给定不可行MIP，找最小不可行子系统（IIS）
  - 可移除线性约束
  - 可移除变量界约束
  - 可放松整数性约束
- **输出**: JSON格式，指定哪些元素放松（relax）或保留（keep）
- **评价**: 子系统大小、计算速度、是否不可约（IIS）

## S-wind实际能力（ARCHITECTURE.md验证）

根据代码跟踪文档`ARCHITECTURE.md`，S-wind的**已实现**能力：

1. **序列公式拟合** (`Fate::explore()`): 
   - 输入：有理数序列
   - 输出：递推关系、封闭形式、代数恒等式等
   - 已知序列检查（OEIS）、差分表、比值、周期检测

2. **精确有理数运算** (`EPR`): 无限精度有理数，无浮点误差

3. **模式库匹配**: 固定模式类（常差分/Newton、比值、积、周期、阶乘、2阶递推）

4. **自循环探索** (`explore_with_self_loop()`):
   - 对未识别序列施加固定变换链（Δ, Δ², Σ, |·|, n·f(n), ratio, f², sgn, MA(3), Δ³, 成对积, mod 10）
   - 闭包检测（所有变换穷举后停止）

5. **验证路径** (`explore_with_verification()`): DoF检查、外推一致性、置信度评估

**未证实能力**:
- ❌ 通用算法综合器
- ❌ Python代码生成器（未在代码中发现）
- ❌ MIP求解或约束分析
- ❌ 组合搜索策略产生

**现有87个bin/是**研究脚本，主要用途：
- 调用`Fate`引擎处理物理信号、工业参数
- 使用`quad`模块的4个有理数物理信号生成器（作为演示源，不在Fate循环内）
- 使用`search`/`conjecture*`模块进行常数搜索和猜想验证（与Fate独立）

**结论**: S-wind是序列公式发现工具，不是通用算法生成系统。

## 实验设计

### 路径探索

本实验诚实探索两条可能路径，并记录真实结果：

#### 路径A：启发式参数序列拟合（探索性）

**假设**: 约束删除优先级权重可能呈现某种数值模式，S-wind可识别并推荐参数序列。

**方法**:
1. 实现简单确定性基线（贪心约束删除）
2. 在小型开发集上尝试不同优先级权重
3. 记录成功率、子系统大小序列
4. 将这些序列输入S-wind `Fate::explore()`
5. 如果S-wind发现模式，用其预测下一组权重
6. 在留出集上验证
7. 导出为独立Python（无S-wind运行时依赖）

**诚实性保证**:
- 如果序列不呈现可拟合模式 → 如实报告
- 如果S-wind未产生有用候选 → 如实报告
- 不将手工设计的启发式冒充为S-wind生成

#### 路径B：基线对比+独立Python实现（保底）

即使路径A失败，交付：
1. 可工作的确定性基线（简单贪心删除）
2. 独立Python实现（使用PuLP/HiGHS）
3. 通过官方checker的合法输出
4. 明确记录S-wind不适用的证据

### 约束条件

1. **本地实验only**: 不向竞赛组织者提交、注册或通信
2. **授权材料only**: 仅使用官方公开的样例和checker
3. **无商业求解器**: 仅用开源工具（PuLP, HiGHS, Python标准库）
4. **独立Python**: 最终代码可在不装S-wind的干净环境运行
5. **时间边界**: 整体实验约10-15分钟小规模测试，不展开长时竞赛搜索

## 目录结构

```
experimental/land_doig_2027/
├── README.md                  # 本文档
├── baseline.py                # 确定性基线实现
├── swind_integration.py       # S-wind调用尝试（如适用）
├── independent_solver.py      # 独立Python最终实现
├── test_runner.py             # 测试和评估脚本
├── requirements.txt           # Python依赖
├── dev_set/                   # 开发集（从smallset选取）
├── holdout_set/               # 留出集（未参与调参）
├── results/                   # 输出JSON和日志
└── RESULTS.md                 # 实验结果报告（中文）
```

## 下一步

1. 安装官方checker
2. 选择dev/holdout分割
3. 实现基线
4. 尝试S-wind集成
5. 记录真实结果
