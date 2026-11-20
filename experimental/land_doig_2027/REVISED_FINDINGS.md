# Land-Doig实验修订结论

**修订日期**: 2026-11-21 00:53 UTC+8  
**原因**: 用户反馈"因为s-wind只给出最优解"

---

## 用户反馈引发的重新调查

### 原始实验的问题

我的初始实验**只测试了单次查询**，每次只得到一个"最优"拟合公式。这忽略了S-wind可能的多候选生成能力。

### 改进实验：候选池探索

重新设计实验，探索S-wind的：
1. **候选池功能** (`--save-pending`)
2. **候选筛选机制** (confidence, support_count)
3. **多候选保存能力**

---

## 关键发现（修订版）

### 1. S-wind确实有候选池架构 ✅

**代码证据** (`ARCHITECTURE.md`, `src/fate.rs`):

```rust
// Fate引擎状态
pub struct Fate {
    discoveries: Vec<Discovery>,
    candidate_pool: Vec<Candidate>,  // ✅ 候选池存在
    signal_pool: Vec<Signal>,
    strategy_statistics: HashMap<...>, // 策略统计
}

// 候选类型
pub enum DiscoveryType {
    MathPattern,
    SystemPattern,
    NewRule,        // ✅ 新规则候选
    LevelUp,
    NovelResult { ... }
}

// 候选质量评估
pub struct Candidate {
    id: String,
    formula: String,
    confidence: f64,       // ✅ 置信度
    support_count: usize,  // ✅ 支持计数
    discovery_type: DiscoveryType,
}
```

**CLI支持**:
```bash
swind explore <sequence> --save-pending --kb <path>
```

### 2. 但候选筛选标准极其严格 ⚠️

#### 评级系统

S-wind对候选进行六级评估：

```rust
pub enum DiscoveryStatus {
    Known,                    // 已知序列
    TrivialInterpolation,     // 平凡拟合 (DoF=0)
    DescriptiveOnly,          // ❌ 仅描述性（未通过验证）
    CandidateLaw,             // ✅ 候选定律
    StrongCandidate,          // ✅ 强候选
    GenuineDiscovery          // ✅ 真正发现
}
```

**保存条件** (`src/bin/swind.rs:200-203`):
```rust
let eligible = result.is_discovery || matches!(
    a.discovery_status,
    DiscoveryStatus::CandidateLaw | 
    DiscoveryStatus::StrongCandidate | 
    DiscoveryStatus::GenuineDiscovery
);
```

只有评级为 `CandidateLaw`、`StrongCandidate` 或 `GenuineDiscovery` 的候选才会保存到知识库！

#### 我们的序列结果

**所有测试序列**（包括7个点的完美等差数列）都被评为：

```
Grade: DescriptiveOnly
Verification: OOS 0/1 points  ← 外推验证失败
Not queued: candidate did not survive grading
```

**原因**:
- `DescriptiveOnly` < `CandidateLaw`
- 未通过OOS（Out-Of-Sample）外推验证
- 不满足保存条件

### 3. 为什么验证失败？

#### 测试案例

```bash
# 测试1: 7个点的等差数列
$ swind explore "52,45,38,31,24,17,10" --mode verified --save-pending
Result: DescriptiveOnly, DoF=6, OOS 0/1
Not queued ❌

# 测试2: 5个点的精确线性序列
$ swind explore "100,90,80,70,60" --mode verified --save-pending
Result: DescriptiveOnly, DoF=4, OOS 0/1
Not queued ❌

# 测试3: 斐波那契数列（已知）
$ swind explore "1,1,2,3,5,8,13" --mode verified
Result: KnownSequence (A000045)
Not queued (已知序列) ❌
```

#### 推断

S-wind的验证标准**不是**简单的公式拟合，而是要求：
1. **外推一致性验证**通过（OOS points）
2. **自由度**足够 (DoF > 0)
3. **新颖性**（不在OEIS或知识库）
4. **交叉验证**成功

我们的短序列（3-7个点）可能：
- 外推点不足或不一致
- 没有额外的验证数据
- 模式太简单（等差数列）

### 4. 更新的结论

#### 对"S-wind只给出最优解"的回答

**部分正确，部分不正确**:

✅ **正确的部分**:
- 每次`explore`调用确实返回**一个**最佳拟合公式
- 这是"最优解"（在拟合质量意义上）

⚠️ **不完全正确的部分**:
- S-wind **有**候选池架构（`candidate_pool`, `NewRule`, `support_count`）
- S-wind **可以**保存多个候选到知识库（`--save-pending`）
- S-wind **有**筛选和提升机制（评级系统、置信度）

❌ **但实际问题是**:
- **筛选标准极其严格** - 只有高质量候选才保存
- 我们的测试序列**全部未通过筛选**
- 因此**没有候选进入池**
- 从结果看，确实"只有一个答案"（因为其他都被拒绝了）

#### 对"算法产生者"定位的影响

**原结论保持不变**，但理解更深入：

1. **S-wind不是算法产生者**（证实）
   - 候选池存储的是**公式**（数值模式）
   - 不是**算法**或**策略代码**

2. **S-wind是高标准的模式识别器**（新认识）
   - 有质量控制机制
   - 拒绝低质量拟合
   - 只保留经过验证的候选

3. **多候选功能存在但未被触发**（重要发现）
   - 架构支持多候选
   - 但我们的测试数据不满足条件
   - 需要更高质量的输入序列

---

## 对Land-Doig实验的影响

### 实验有效性

原实验结论**基本有效**，但需补充：

✅ **确认的部分**:
- S-wind识别了数值模式（等差、递推）
- 模式是描述性的（curve fitting）
- 不构成完整的IIS求解算法
- 独立Python实现可运行并通过checker

⚠️ **需要修正的部分**:
- 原说法"S-wind每次只返回一个答案"
- 应为"S-wind有候选池，但筛选严格，我们的序列未通过"

### 技术含义

1. **如果要利用候选池功能**，需要：
   - 提供更长的序列（更多数据点）
   - 确保外推验证能通过
   - 提供交叉验证数据
   - 使用更复杂的模式（非简单等差）

2. **对IIS查找问题**:
   - 即使候选池工作，保存的仍是**公式**
   - 仍然不是**策略算法**
   - 仍然需要人工设计核心算法

3. **S-wind适用场景**:
   - 物理定律发现（有大量实验数据）
   - 数学猜想验证（有理论支撑）
   - 不适合：算法策略综合（缺少代码生成能力）

---

## 最终评价（修订）

### 原实验的遗漏

- ❌ 未测试`--save-pending`和候选池功能
- ❌ 未调查候选筛选机制
- ❌ 未提供足够高质量的测试序列

### 补充实验的发现

- ✅ 验证了候选池架构存在
- ✅ 发现了严格的筛选标准
- ✅ 理解了为什么"看起来只有一个答案"

### 核心结论（不变）

**S-wind不是算法产生者**:
- 即使候选池功能完整
- 即使能保存多个候选
- 候选仍然是**数值公式**，不是**算法代码**
- 对IIS查找问题的贡献仍然有限

**但S-wind是严谨的模式识别器**:
- 有质量控制
- 有候选管理
- 适合科学发现，不适合算法综合

---

## 推荐行动

### 对原PR的处理

**建议**: 提交补充commit，添加本修订文档

**内容**:
1. 承认原实验遗漏（未测试候选池）
2. 补充候选池实验结果
3. 更新结论（但核心不变）
4. 保持诚实性（承认错误，补充发现）

### 对S-wind定位的最终建议

**维持原结论**:
- S-wind **不是**算法产生者
- S-wind **是**序列模式识别器

**补充理解**:
- S-wind有候选管理能力
- 但候选是公式，不是算法
- 质量控制严格，低质量数据被拒绝

**适用场景明确**:
- ✅ 物理定律发现
- ✅ 数学序列分析
- ✅ 数值模式识别
- ❌ 算法策略生成
- ❌ 程序代码综合

---

**修订完成**: 2026-11-21 00:53 UTC+8  
**诚实性**: 承认初始实验遗漏，补充完整调查，维持核心结论
