# S-wind User Manual

> **S-wind**: Formal Algebraic Exploration System  
> **Version**: 1.1.0 (Self-Loop Enabled)  
> **Core Principle**: All computation over ℚ (EPR - Extended Precision Rational), no f64, unbounded precision

---

## 1. 概述

S-wind 是一个精确代数模拟框架，所有计算使用 Rational 有理数，零浮点误差。

### 核心模块

| 模块 | 功能 |
|------|------|
| `symbolic` | Rational 算术、Expr 符号表达式、to_latex() |
| `knowledge` | KnowledgeBase 知识库 |
| `fate` | Fate 统一探索引擎 (含自循环) |
| `quad` | 4 大物理系统 |
| `space` | 空间容器、分形维度计算 |
| `voyager` | 反推验证、完整推导 |

---

## 2. 快速开始

```rust
use s_wind::{EPR, Fate, KnowledgeBase};

fn main() {
    // 精确有理数 (EPR = Extended Precision Rational)
    let r = EPR::from_ratio(1, 3);  // 1/3 精确表示
    
    // Fate 探索引擎
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    let mut fate = Fate::new();
    
    let signal = vec![EPR::from_i128(1), EPR::from_i128(2), EPR::from_i128(3)];
    let result = fate.explore(&signal, &kb);
    
    println!("Status: {}", result.status.label());
    if let Some(formula) = result.formula {
        println!("Formula: {}", formula);
    }
}
```

---

## 3. 核心接口

### 3.1 Rational 有理数

```rust
use s_wind::symbolic::Rational;

let a = Rational::new(1, 2);   // 1/2
let b = Rational::new(2, 3);   // 2/3
let c = a + b;                  // 7/6 (精确)
let d = a * b;                  // 1/3 (精确)

println!("{}", c);  // 输出: 7/6
```

### 3.2 Expr 符号表达式

```rust
use s_wind::symbolic::{Expr, Symbol};

// 符号
let x = Expr::sym("x");
let n = Expr::sym("n");

// 幂次
let x_sq = Expr::Pow(Box::new(x.clone()), Box::new(Expr::num(2)));
println!("{}", x_sq.to_latex());  // x^{2}

// 求和
let sum = Expr::Sum {
    var: Symbol::new("k"),
    start: Box::new(Expr::num(1)),
    end: Box::new(n.clone()),
    body: Box::new(Expr::Div(
        Box::new(Expr::num(1)),
        Box::new(Expr::Pow(Box::new(Expr::sym("k")), Box::new(Expr::num(2))))
    )),
};
println!("{}", sum.to_latex());  // \sum_{k=1}^{n} \frac{1}{k^{2}}
```

### 3.3 数学函数

```rust
use s_wind::{basel_partial_sum, harmonic_number};

// Basel 部分和
let sum_100 = basel_partial_sum(100);
println!("Σ 1/k² (k=1..100) = {}", sum_100);

// 调和数
let h_10 = harmonic_number(10);
println!("H_10 = {}", h_10);
```

---

## 4. Triple-Fate 自主探索

### 4.1 基本用法

```rust
use s_wind::{Fate, KnowledgeBase, EPR, DiscoveryType};

let mut kb = KnowledgeBase::new();
kb.initialize_fundamentals();
let mut fate = Fate::new();

// 输入信号进行探索
let signal = vec![EPR::from_i128(1), EPR::from_i128(2), EPR::from_i128(3)];
let result = fate.explore(&signal, &kb);

// 检查结果
println!("Status: {}", result.status.label());
println!("Formula: {:?}", result.formula);
println!("Is Discovery: {}", result.is_discovery);

// 自循环探索 (推荐用于短序列)
let (result, iterations, stats) = fate.explore_with_self_loop(&signal, &kb);
println!("Self-loop stats: {}", stats.summary());
```

### 4.2 Discovery 输出

```
ID: LEVEL_UP_36
描述: ★ LEVEL-UP: Created new meta-system
数学推导: Level-Up triggered by combined evolution:
  • Base Fate generation: 1 (threshold: ≥1)
  • Meta Fate generation: 1 (threshold: ≥1)
  • Formula: (base.gen ≥ 1 ∧ meta.gen ≥ 1) ∨ (meta_count > 1000)
触发条件: CONDITION A: Both Base(1) and Meta(1) evolved
```

---

## 5. Voyager 反推验证

### 5.1 基本用法

```rust
use voyager::{PreciseSignalDetector, Rational};

let mut detector = PreciseSignalDetector::new();

// 输入信号
let signal = vec![
    Rational::new(1, 2),
    Rational::new(1, 1),
    Rational::new(3, 2),
];

// 检测模式
let result = detector.detect_signal_precisely(signal.clone());

// 获取推导
if let Some(trace) = detector.format_derivation_trace("eq_0") {
    println!("{}", trace);
}
```

### 5.2 推导输出示例

```
Theorem (eq_1)
Let f(n) = 1/2·n + -1/2.

Proof.
  (1) S = {1/2, 1, 3/2}  by axiom
  (2) s_1 = 1/2  by definition
  (3) s_2 = 1  by definition
  (4) s_3 = 3/2  by definition
  (5) |S| = 3  by axiom
  (6) Compute Δ¹ = {Δ¹₁, Δ¹₂, ...}  by definition
  (7) Δ¹_1 = 1/2  by subtraction
  (8) Δ¹_2 = 1/2  by subtraction
  (9) ∀i: Δ¹ᵢ = 1/2 (constant)  by simplification
  (10) Compute Δ² = {Δ²₁, Δ²₂, ...}  by definition
  (11) Δ²_1 = 0  by subtraction
  (12) ∀i: Δ²ᵢ = 0 → Linear pattern confirmed  by simplification
  ...
  (35) ∎  by axiom

  ∴ The equation is algebraically verified.  ∎
```

---

## 6. Quad 物理系统

### 6.1 四大系统

```rust
use s_wind::quad::{
    GravitationalField, GravitationalFieldConfig,
    QuantumFieldSystem, QuantumFieldConfig,
    ThermalRadiationProcessor, ThermalRadiationConfig,
    FundamentalCouplingCoordinator,
};
use s_wind::symbolic::Rational;

// 引力场
let gravity = GravitationalField::new(GravitationalFieldConfig::default());
let force = gravity.gravitational_force(Rational::new(141347, 10000));

// 量子场
let mut quantum = QuantumFieldSystem::new(QuantumFieldConfig::default());
let signal = quantum.generate_signal(Rational::new(1, 1), [Rational::zero(); 3]);
println!("量子场能量: {}", signal.mode_energy);

// 热辐射
let mut thermal = ThermalRadiationProcessor::new(ThermalRadiationConfig::default());
let event = thermal.process_radiation(Rational::new(1, 1));
println!("supercritical: {}", event.is_supercritical);

// 基本耦合
let mut coupling = FundamentalCouplingCoordinator::new();
let result = coupling.coordinate(
    Rational::new(1, 1),
    Rational::new(1, 1),
    Rational::new(1, 1),
);
println!("G:{} E:{} S:{}", 
    result.gravitational_active,
    result.electromagnetic_active,
    result.strong_active);
```

---

## 7. Space 空间与维度

```rust
use s_wind::space::{SWindSpace, FractionalDimensionCalculator, ContainerType};
use s_wind::symbolic::Rational;

// 创建空间
let mut space = SWindSpace::new();
space.create_container(ContainerType::Basic);
space.create_container(ContainerType::Adaptive);

// 分形维度计算
let mut dim_calc = FractionalDimensionCalculator::new();
let data: Vec<Rational> = (1..=10).map(|i| Rational::new(i, 1)).collect();
let result = dim_calc.calculate_optimal_dimension(&data);

println!("推荐维度: {}", result.recommended_dimension);
println!("维度类型: {:?}", result.dimension_type);

// 自适应维度扩展
let adaptive_dim = space.adaptive_dimension_expansion(&data);
println!("自适应维度: {}", adaptive_dim);
```

---

## 8. KnowledgeBase 知识库

```rust
use s_wind::knowledge::KnowledgeBase;

let mut kb = KnowledgeBase::new();
kb.initialize_fundamentals();

println!("物理常数: {} 个", kb.physical_constants.len());
println!("定理: {} 个", kb.theorems.len());
println!("物理定律: {} 个", kb.physical_laws.len());

// 添加发现
let id = kb.add_discovery(
    r"\sum_{k=1}^{n} \frac{1}{k^2} \to \frac{\pi^2}{6}".into(),
    "Basel problem".into(),
    "S-wind exact computation".into(),
);
println!("新发现 ID: {}", id);
```

---

## 9. 运行实验

### 9.1 命令行

```bash
# 前沿实验
cargo run --bin frontier_experiments --release

# 高价值实验
cargo run --bin high_value_experiments --release

# 反推验证示例
cargo run --example reverse_verification --release

# 生成输出文件
cargo run --bin generate_output --release
```

### 9.2 输出文件

```
s-wind-sim/output/
├── {problem}_answer_{timestamp}.tex      # 答案
├── {problem}_reverse_{timestamp}.tex     # 反推验证
└── {problem}_derivation_{timestamp}.tex  # 详细推导
```

---

## 10. 核心原则

| 原则 | 说明 |
|------|------|
| **EPR 全程** | 所有计算使用 EPR (Extended Precision Rational) 无限精度 |
| **无 f64** | 禁止浮点数，零舍入误差 |
| **精确推导** | Voyager 输出完整代数证明 |
| **自主探索** | Fate 系统自主决定探索深度 |
| **无编造** | 输出只来自 S-wind 接口 |

---

## 11. 接口参考

### Expr.to_latex()
```rust
let expr = Expr::Pow(Box::new(Expr::sym("n")), Box::new(Expr::num(2)));
expr.to_latex()  // "n^{2}"
```

### ExplorationResult.derivation_steps
```rust
let result = fate.explore(&signal, &kb);
for step in result.derivation_steps {
    println!("{}", step);  // S-wind 自动生成的推导步骤
}
```

### SelfLoopStats.summary()
```rust
let (result, iterations, stats) = fate.explore_with_self_loop(&signal, &kb);
println!("{}", stats.summary());  // "Iterations: 3, Signal: 3→18, DoF Δ: +10"
```

### voyager.format_derivation_trace()
```rust
detector.format_derivation_trace("eq_0")  // 完整推导步骤
```

### KnowledgeBase.add_discovery()
```rust
kb.add_discovery(equation_tex, description, derivation_tex)
```

---

*S-wind: Complete Exact Algebraic Simulation*
