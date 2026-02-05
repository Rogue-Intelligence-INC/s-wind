# S-wind Algebraic Exhaustive Exploration Report

**Date**: 2026-02-01  
**Mode**: Pure EPR (Extended Precision Rational)  
**Feature**: No floating point - Exact computation only

---

## 1. Derivation Verification (from program output)

### 1.1 Polynomial P(D) Step-by-Step Derivation

**Definition**:
```
P(D) = D^4 - 2D^3 + 2D + 1
```

**Step-by-step evaluation at D = 4**:
```
Step 1: D^4 = 4^4 = 4 × 4 × 4 × 4 = 256
Step 2: D^3 = 4^3 = 4 × 4 × 4 = 64
Step 3: 2D^3 = 2 × 64 = 128
Step 4: 2D = 2 × 4 = 8
Step 5: Constant term = 1

P(4) = 256 - 128 + 8 + 1
     = 128 + 8 + 1
     = 137  ✓ (EXACT INTEGER)
```

**Program Verification**:
```
## Container 1: Dimension Polynomial

  P(4) = D^4 - 2D^3 + 2D + 1 at D=4
  Computed: P(4) = 137
  Target: 137
  Exact match: YES (algebraically exact)
```

### 1.2 Exact Difference from α⁻¹ (EPR Computation)

**Input**:
```
α⁻¹ = 137.035999084 (CODATA 2018)
```

**Conversion to exact rational**:
```
α⁻¹ = 137035999084 / 1000000000
    = 34258999771 / 250000000  (after GCD reduction)
```

**Exact subtraction**:
```
α⁻¹ - 137 = 34258999771/250000000 - 137/1
          = 34258999771/250000000 - 34250000000/250000000
          = (34258999771 - 34250000000) / 250000000
          = 8999771 / 250000000
          ≈ 0.0359991
```

**Program Verification**:
```
## Container 3: Alpha Inverse Analysis

  α⁻¹ = 34258999771/250000000
  137 = 137
  Difference: α⁻¹ - 137 = 8999771/250000000
  Exact equality: NO
```

### 1.3 P(D) Evaluation for D = 1 to 12 (with derivation)

| D | D⁴ | 2D³ | 2D | P(D) = D⁴ - 2D³ + 2D + 1 | Prime? | Factorization |
|---|-----|-----|-----|---------------------------|--------|---------------|
| 1 | 1 | 2 | 2 | 1 - 2 + 2 + 1 = **2** | Yes | 2 |
| 2 | 16 | 16 | 4 | 16 - 16 + 4 + 1 = **5** | Yes | 5 |
| 3 | 81 | 54 | 6 | 81 - 54 + 6 + 1 = **34** | No | 2 × 17 |
| 4 | 256 | 128 | 8 | 256 - 128 + 8 + 1 = **137** | Yes | 137 |
| 5 | 625 | 250 | 10 | 625 - 250 + 10 + 1 = **386** | No | 2 × 193 |
| 6 | 1296 | 432 | 12 | 1296 - 432 + 12 + 1 = **877** | Yes | 877 |
| 7 | 2401 | 686 | 14 | 2401 - 686 + 14 + 1 = **1730** | No | 2 × 5 × 173 |
| 8 | 4096 | 1024 | 16 | 4096 - 1024 + 16 + 1 = **3089** | Yes | 3089 |
| 9 | 6561 | 1458 | 18 | 6561 - 1458 + 18 + 1 = **5122** | No | 2 × 13 × 197 |
| 10 | 10000 | 2000 | 20 | 10000 - 2000 + 20 + 1 = **8021** | No | **13 × 617** |
| 11 | 14641 | 2662 | 22 | 14641 - 2662 + 22 + 1 = **12002** | No | 2 × 17 × 353 |
| 12 | 20736 | 3456 | 24 | 20736 - 3456 + 24 + 1 = **17305** | No | 5 × 3461 |

**Program Verification**:
```
## Primality Pattern Analysis

  D  | P(D)  | Prime? | Factorization
  ---|-------|--------|---------------
   1 |     2 | Yes    | 2
   2 |     5 | Yes    | 5
   3 |    34 | No     | 2 × 17
   4 |   137 | Yes    | 137
   5 |   386 | No     | 2 × 193
   6 |   877 | Yes    | 877
   7 |  1730 | No     | 2 × 5 × 173
   8 |  3089 | Yes    | 3089
   9 |  5122 | No     | 2 × 13 × 197
  10 |  8021 | No     | 13 × 617
  11 | 12002 | No     | 2 × 17 × 353
  12 | 17305 | No     | 5 × 3461
```

**Key Finding**: P(D) is prime for D ∈ {1, 2, 4, 6, 8} but **NOT** for D = 10, 12.
Hypothesis "P(even D) is always prime" is **FALSIFIED**.

---

## 2. Exhaustive Polynomial Search Results

### 2.1 Search Algorithm

For degree-4 polynomial: `Q(D) = a₀ + a₁D + a₂D² + a₃D³ + a₄D⁴`

At D = 4:
```
Q(4) = a₀ + 4a₁ + 16a₂ + 64a₃ + 256a₄ = 137
```

Given (a₁, a₂, a₃, a₄), solve for a₀:
```
a₀ = 137 - 4a₁ - 16a₂ - 64a₃ - 256a₄
```

Count solutions where a₀ is within coefficient range.

### 2.2 Results

| Coefficient Range | Search Space | Solutions |
|-------------------|--------------|-----------|
| [-5, 5] | 11⁴ = 14,641 | **61** |
| [-10, 10] | 21⁴ = 194,481 | **716** |

**Program Verification**:
```
================================================================================
          EXHAUSTIVE ALGEBRAIC POLYNOMIAL SEARCH
================================================================================
  Max degree: 4
  Coefficient range: [-5, 5]
  Evaluation point: D = 4
  Target: 137
================================================================================

Search complete:
  Total polynomials yielding 137 at D=4: 61
  Exact matches: 61

================================================================================
          EXTENDED EXHAUSTIVE SEARCH (coefficients in [-10, 10])
================================================================================

  Coefficient range: [-10, 10]
  Search space: 21^4 = 194481 coefficient combinations
  Polynomials yielding P(4) = 137: 716

  This proves NON-UNIQUENESS: 715 alternatives exist.
```

---

## 3. Koide Formula Verification

### 3.1 Step-by-Step Derivation

**Lepton masses (PDG 2022)**:
```
m_e  = 0.51099895 MeV
m_μ  = 105.6583755 MeV
m_τ  = 1776.86 MeV
```

**Square roots**:
```
√m_e  = √0.51099895  = 0.714841906
√m_μ  = √105.6583755 = 10.279026000
√m_τ  = √1776.86     = 42.152817225
```

**Koide Q computation**:
```
Σ√m = √m_e + √m_μ + √m_τ 
    = 0.714841906 + 10.279026000 + 42.152817225
    = 53.146685130

(Σ√m)² = 53.146685130² = 2824.570140321

Σm = m_e + m_μ + m_τ
   = 0.51099895 + 105.6583755 + 1776.86
   = 1883.029374450

Q = Σm / (Σ√m)²
  = 1883.029374450 / 2824.570140321
  = 0.666660511
```

**Comparison with target**:
```
Q_computed = 0.666660511
Q_target   = 2/3 = 0.666666667
Deviation  = |0.666660511 - 0.666666667| / 0.666666667 × 100%
           = 0.000923%
```

**Program Verification**:
```
================================================================================
          KOIDE FORMULA EXACT VERIFICATION
================================================================================

  Lepton masses (PDG 2022):
    m_e  = 0.51099895 MeV
    m_μ  = 105.6583755 MeV
    m_τ  = 1776.86 MeV

  Koide Q computation (step-by-step):
    √m_e  = 0.714841906
    √m_μ  = 10.279026000
    √m_τ  = 42.152817225
    Σ√m   = 53.146685130
    (Σ√m)² = 2824.570140321
    Σm    = 1883.029374450
    Q = Σm / (Σ√m)² = 0.666660511
    Target: 2/3 = 0.666666667
    Deviation: 0.000923%

  Status: ✓ VERIFIED to <0.01% (remarkable agreement)
```

---

## 4. Cross-Domain Search Results

### 4.1 Weinberg Angle (Tree-Level)

```
sin²θ_W (measured) = 0.23122
1 - (M_W/M_Z)² = 0.223059
Relative error: 3.53%
Status: Tree-level relation (radiative corrections needed)
```

### 4.2 Cosmological Ratio

```
Ω_Λ / Ω_m = 6889/3111 ≈ 2.2144
Closest simple ratio: 7/3 = 2.3333
Error from 7/3: 5.37%
Status: No exact simple fraction match
```

### 4.3 Golden Ratio Test

```
α⁻¹ ≈ 137.036
φ^10 ≈ 122.99
φ^11 ≈ 199.00
Status: No match (137 is not a power of φ)
```

---

## 5. Summary Table

| Claim | Status | Evidence |
|-------|--------|----------|
| P(4) = 137 | **✓ VERIFIED** | Algebraic identity: 256 - 128 + 8 + 1 = 137 |
| P(1) = F₃ | **✓ VERIFIED** | P(1) = 2 = F₃ (Fibonacci) |
| Koide Q = 2/3 | **✓ VERIFIED** | 0.666660511, deviation 0.0009% |
| sin²θ_W ≈ 1-(M_W/M_Z)² | **≈ VERIFIED** | Tree-level, 3.5% off (needs corrections) |
| P(even D) always prime | **✗ FALSIFIED** | P(10) = 13 × 617 |
| P(D) uniqueness | **✗ FALSIFIED** | 716 alternatives (coeff ∈ [-10,10]) |
| Ω_Λ/Ω_m = simple fraction | **✗ NOT FOUND** | 5.4% off from 7/3 |
| α⁻¹ = φⁿ | **✗ NOT FOUND** | Not a power of golden ratio |
| Multi-constant constraint | **✗ NOT FOUND** | No formula constrains multiple constants |

---

## 6. Final Conclusion

```
================================================================================
                         FINAL ALGEBRAIC CONCLUSION
================================================================================

All computations performed with EPR (Extended Precision Rational):
  - No floating point errors (except where √ required)
  - Exact arithmetic for all integer/rational operations
  - Algebraically verifiable

KEY RESULTS:

1. ALGEBRAIC IDENTITY (EXACT):
   P(4) = D^4 - 2D^3 + 2D + 1 at D=4 = 256 - 128 + 8 + 1 = 137

2. NON-UNIQUENESS (EXACT):
   - 61 polynomials (coeff in [-5,5]) yield P(4) = 137
   - 716 polynomials (coeff in [-10,10]) yield P(4) = 137

3. FALSIFIED HYPOTHESES:
   - 'P(even D) is always prime' → FALSE at D=10, P(10) = 13 × 617
   - 'P(D) is unique' → FALSE, many alternatives exist

4. VERIFIED RELATIONS:
   - Koide Q = 2/3 (to 0.001%)
   - sin²θ_W ≈ 1 - (M_W/M_Z)² (tree-level, needs corrections)

5. NOT FOUND:
   - Multi-constant algebraic constraints
   - Exact simple-fraction fits to cosmological ratios
   - α⁻¹ as power of φ (golden ratio)

CONCLUSION:
  The identity P(4) = 137 is mathematically exact but physically uninterpreted.
  No unified constraint linking multiple fundamental constants was discovered.
```

---

## Appendix: Source Code

**File**: `s-wind-sim/src/bin/algebraic_exhaustive_exploration.rs`

**Run command**:
```bash
cd s-wind-sim && cargo run --bin algebraic_exhaustive_exploration
```
