# S-wind: Autonomous Algebraic Discovery System

[![Status](https://img.shields.io/badge/Status-Alpha-red.svg)]()
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> ⚠️ **ALPHA TEST** - This software is under active development. APIs may change without notice.

**S-wind** is an autonomous discovery system that performs exhaustive algebraic exploration over the field of rational numbers ℚ. It uses Extended Precision Rationals (EPR) for exact arithmetic—no floating-point approximations.

---

## Key Features

- **EPR (Extended Precision Rationals)**: All computation is exact over ℚ
- **Exhaustive Exploration**: Bounded closure certification proves search completeness
- **Self-Loop Discovery**: Patterns extend signals for deeper verification
- **Automatic LaTeX Generation**: Derivations output directly to papers

---

## Quick Start

### Installation

```bash
git clone https://github.com/YOUR_USERNAME/s-wind.git
cd s-wind
cargo build --release
```

### Basic Usage

```rust
use s_wind::{Fate, KnowledgeBase, EPR};

// Create exploration engine
let mut fate = Fate::new();
let kb = KnowledgeBase::new();

// Input signal (exact rationals)
let signal = vec![
    EPR::from(22),
    EPR::from(42),
    EPR::from(122),
];

// Exhaustive exploration with self-loop
let (result, history, stats) = fate.explore_with_self_loop(&signal, &kb);

println!("Discovered: {:?}", result);
// Output: s[n+1] = 4·s[n] - 46 [linear recurrence]
```

### Run Examples

```bash
# Fate exhaustive exploration with LaTeX output
cargo run --release --bin fate_exhaustive_latex

# Algebraic unified theory verification
cargo run --release --bin algebraic_unified_theory

# Frontier physics exploration
cargo run --release --bin frontier_physics_exploration
```

---

## Core Concepts

### 1. EPR (Extended Precision Rational)

All numbers are exact fractions—no floating-point errors:

```rust
use s_wind::symbolic::EPR;

let a = EPR::from_ratio(1, 3);  // 1/3 exactly
let b = EPR::from_ratio(1, 6);  // 1/6 exactly
let sum = a + b;                 // 1/2 exactly (not 0.49999...)
```

### 2. Exhaustive Search with Closure

S-wind guarantees **true exhaustion** through mathematical closure:

```
CLOSURE(E) := ∀τ ∈ T, ∀S ∈ E: sig(τ(S)) ∈ E
```

When no transformation produces new signals, exploration is **provably complete**.

### 3. Self-Loop Discovery

Short signals are extended using detected patterns:

```
Input:  [3, 6, 9]     (3 points, trivial)
   ↓ detect f(n) = 3n
   ↓ extend signal
Output: [3,6,9,12,15,18,21,24,27,30,33]  (11 points, non-trivial)
```

---

## Research Discoveries

S-wind has been used to explore fundamental physics conjectures:

### Unified Hierarchy Conjecture

The sequence `{22, 42, 122}` (mass hierarchy, coupling hierarchy, cosmological constant) satisfies:

$$s_{n+1} = 4s_n - 46$$

Under the ansatz $\Delta_1 = D(D+1)$, spacetime dimension $D=4$ emerges as the unique solution, predicting $\alpha^{-1} = 137$ with 0.026% accuracy.

**See**: `papers/unified_theory/MASTER_PHYSICS_PAPER.tex`

### Fusion Constraint Analysis

The Q-factor follows a Möbius transformation structure:

$$Q = \frac{176L}{176 - 35L}$$

This explains the non-linear difficulty of achieving fusion gain.

**See**: `papers/unified_theory/FUSION_ALGEBRAIC_PAPER.tex`

---

## Project Structure

```
s-wind/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── fate.rs             # Core exploration engine
│   ├── symbolic/
│   │   ├── epr.rs          # Extended Precision Rationals
│   │   └── expr.rs         # Symbolic expressions
│   ├── knowledge/
│   │   ├── mod.rs          # Knowledge base
│   │   └── oeis.rs         # OEIS sequence loader
│   └── bin/
│       ├── fate_exhaustive_latex.rs
│       ├── algebraic_unified_theory.rs
│       └── ...
├── papers/
│   └── unified_theory/
│       ├── MASTER_PHYSICS_PAPER.tex
│       ├── FUSION_ALGEBRAIC_PAPER.tex
│       └── SUPPLEMENTARY_MATERIAL.tex
├── data/
│   └── oeis/               # OEIS b-files
└── output/                 # Generated LaTeX/reports
```

---

## API Quick Reference

### Fate (Exploration Engine)

```rust
let mut fate = Fate::new();

// Standard exploration
let result = fate.explore(&signal, &kb);

// With self-loop (extends signal)
let (result, history, stats) = fate.explore_with_self_loop(&signal, &kb);
```

### EPR (Exact Rationals)

```rust
let x = EPR::from(42);           // Integer
let y = EPR::from_ratio(3, 7);   // Fraction 3/7
let z = x + y;                    // Exact arithmetic
let w = z.pow(10);                // Large powers OK
```

### KnowledgeBase

```rust
let kb = KnowledgeBase::new();
// Contains: OEIS sequences, physical constants, theorems
```

---

## Reproducibility

All discoveries include:
- **EPR verification**: Exact rational arithmetic, no floating-point
- **Closure certificate**: Proof of exhaustive search
- **LaTeX derivation**: Step-by-step output

Example output hash for verification:
```
fate_exhaustive: sha256:7a3b2c1d...
```

---

## Citation

```bibtex
@software{swind2026,
  title = {S-wind: Autonomous Algebraic Discovery System},
  author = {Ma, Zhuoli},
  year = {2026},
  note = {Alpha release},
  organization = {Rogue Intelligence LNC.},
  url = {https://github.com/joulymars/s-wind}
}
```

---

## License

MIT License

---

## Disclaimer

⚠️ **Alpha Test Software**

- This is research software under active development
- APIs and output formats may change
- Physics conjectures are **unverified hypotheses**, not established science
- Use at your own risk

---

*S-wind: Where exactness meets discovery.*
