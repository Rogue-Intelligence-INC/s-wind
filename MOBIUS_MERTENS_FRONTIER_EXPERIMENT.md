# Möbius/Mertens Function: Multi-Scale Structure Analysis

**Date**: 2026-01-24 (Final)  
**System**: S-wind v1.1.1 (Newton Forward Difference Solver)  
**Classification**: Frontier Mathematical Experiment

---

## 1. Problem Statement

**Research Question**: Can S-wind solve for the exact polynomial representation of the normalized Mertens function?

---

## 2. Mathematical Background

### 2.1 Möbius Function μ(n)

$$\mu(n) = \begin{cases} 
1 & \text{if } n = 1 \\
(-1)^k & \text{if } n \text{ is product of } k \text{ distinct primes} \\
0 & \text{if } n \text{ has squared prime factor}
\end{cases}$$

### 2.2 Mertens Function M(n)

$$M(n) = \sum_{k=1}^{n} \mu(k)$$

### 2.3 Normalized Sequence

$$A_n = \left\lfloor \frac{M(n)}{\sqrt{n}} \times 100 \right\rfloor + 100$$

---

## 3. Experimental Data

| n | μ(n) | M(n) | A_n |
|---|------|------|-----|
| 1 | 1 | 1 | 200 |
| 2 | -1 | 0 | 100 |
| 3 | -1 | -1 | 42 |
| 4 | 0 | -1 | 50 |
| 5 | -1 | -2 | 11 |
| 6 | 1 | -1 | 59 |
| 7 | -1 | -2 | 24 |
| 8 | 0 | -2 | 29 |
| 9 | 0 | -2 | 33 |
| 10 | 1 | -1 | 68 |

---

## 4. S-wind Solution (Newton Forward Difference)

### 4.1 Input

```
Signal: [200, 100, 42, 50, 11, 59, 24, 29, 33, 68]
Length: n = 10
```

### 4.2 Newton Forward Difference Coefficients

S-wind computes the forward differences Δᵏ[0]:

| k | Δᵏ | Δᵏ[0] |
|---|-----|-------|
| 0 | [200, 100, 42, 50, 11, 59, 24, 29, 33, 68] | **200** |
| 1 | [-100, -58, 8, -39, 48, -35, 5, 4, 35] | **-100** |
| 2 | [42, 66, -47, 87, -83, 40, -1, 31] | **42** |
| 3 | [24, -113, 134, -170, 123, -41, 32] | **24** |
| 4 | [-137, 247, -304, 293, -164, 73] | **-137** |
| 5 | [384, -551, 597, -457, 237] | **384** |
| 6 | [-935, 1148, -1054, 694] | **-935** |
| 7 | [2083, -2202, 1748] | **2083** |
| 8 | [-4285, 3950] | **-4285** |
| 9 | [8235] | **8235** |

### 4.3 Exact Solution (Newton Form)

**S-wind solved polynomial:**

$$f(n) = 200 + (-100)\binom{n-1}{1} + 42\binom{n-1}{2} + 24\binom{n-1}{3} + (-137)\binom{n-1}{4}$$
$$+ 384\binom{n-1}{5} + (-935)\binom{n-1}{6} + 2083\binom{n-1}{7} + (-4285)\binom{n-1}{8} + 8235\binom{n-1}{9}$$

**Compact form:**

```
f(n) = 200 - 100·C(n-1,1) + 42·C(n-1,2) + 24·C(n-1,3) - 137·C(n-1,4) 
     + 384·C(n-1,5) - 935·C(n-1,6) + 2083·C(n-1,7) - 4285·C(n-1,8) + 8235·C(n-1,9)
```

### 4.4 Verification (Sample Domain)

| n | Computed f(n) | Actual A_n | Match |
|---|---------------|------------|-------|
| 1 | 200 | 200 | ✓ |
| 2 | 200 - 100 = 100 | 100 | ✓ |
| 3 | 200 - 200 + 42 = 42 | 42 | ✓ |
| 4 | 200 - 300 + 126 + 24 = 50 | 50 | ✓ |
| 5 | ... = 11 | 11 | ✓ |
| 6 | ... = 59 | 59 | ✓ |
| 7 | ... = 24 | 24 | ✓ |
| 8 | ... = 29 | 29 | ✓ |
| 9 | ... = 33 | 33 | ✓ |
| 10 | ... = 68 | 68 | ✓ |

**Sample domain: 10/10 exact matches** ✓

---

## 5. Cross-Domain Verification

### 5.1 OOS Test

```
n = 11: f(11) = 15103 (predicted by Newton formula)
Actual: A₁₁ = floor(M(11)/√11 × 100) + 100 = floor(-2/3.317 × 100) + 100 ≈ 40

Discrepancy: |15103 - 40| = 15063
```

### 5.2 Result

```
OOS Verification: ✗ FAILED
Reason: Polynomial extrapolation diverges from true Möbius behavior
```

---

## 6. Fit Analysis (Formal DoF)

```
n (sample size): 10
d (degree): 9
dim(Θ_H) (parameters): 10

DoF(H, n) = n - dim(Θ_H) = 10 - 10 = 0

is_trivial := (DoF = 0): true
ρ (non-triviality ratio): 0/1 = 0
```

---

## 7. Complete Derivation (34 Steps)

```
(1)  Test pattern: Reciprocal
(2)  Test pattern: Exponential
(3)  Test pattern: Periodic
(4)  Test pattern: Factorial
(5)  Test pattern: Constant
(6)  Test pattern: Linear Polynomial
(7)  Test pattern: Quadratic Polynomial
(8)  Test pattern: Cubic Polynomial
(9)  Test pattern: Linear Recurrence (order 2)
(10) Test pattern: Linear Recurrence (order 3)
(11) Test pattern: General Polynomial
(12) Input: [200, 100, 42, 50, 11, 59, 24, 29, 33, 68]
(13) Δ¹ = [-100, -58, 8, -39, 48, -35, 5, 4, 35]
(14) Δ² = [42, 66, -47, 87, -83, 40, -1, 31]
(15) Δ³ = [24, -113, 134, -170, 123, -41, 32]
(16) Δ⁴ = [-137, 247, -304, 293, -164, 73]
(17) Δ⁵ = [384, -551, 597, -457, 237]
(18) Δ⁶ = [-935, 1148, -1054, 694]
(19) Δ⁷ = [2083, -2202, 1748]
(20) Δ⁸ = [-4285, 3950]
(21) Δ⁹ = [8235]
(22) ✓ Δ⁹ constant = 8235
(23) Formula: f(n) = 200 - 100·C(n-1,1) + 42·C(n-1,2) + ... + 8235·C(n-1,9)
(24) NEWTON FORWARD DIFFERENCE THEOREM applied
(25) Polynomial degree 9 (determined by Δ⁹ = constant)
(26) Sample domain verification: 10/10 ✓
(27) ✗ OOS FAILED at n=11: predicted 15103, actual ~40
(28) ❌ NOT DISCOVERY: Cross-domain extrapolation failed
(29-34) Adaptive cross-domain verification details
```

---

## 8. Mathematical Interpretation

### 8.1 What S-wind Solved

S-wind successfully computed the **unique polynomial of degree ≤ 9** passing through all 10 points. This is guaranteed by:

**Lagrange Interpolation Theorem**: For n distinct points, there exists a unique polynomial of degree ≤ n-1.

### 8.2 Why OOS Failed

The Möbius function is **not a polynomial**. It is:
- Defined by prime factorization
- Multiplicative function
- Highly oscillatory

The degree-9 polynomial is an **exact interpolant** on [1,10] but has **zero predictive power** outside this domain.

### 8.3 Conclusion

| Aspect | Result |
|--------|--------|
| **Solved** | ✓ Exact degree-9 polynomial in Newton form |
| **Verified (sample)** | ✓ 10/10 points |
| **Extrapolation** | ✗ Failed |
| **Discovery** | ❌ No (interpolation, not structure) |

---

## 9. Newton Forward Difference Formula

### Theorem (Added to KnowledgeBase)

$$f(n) = \sum_{k=0}^{d} \binom{n-1}{k} \Delta^k f(1)$$

where:
$$\Delta^0 f(i) = f(i)$$
$$\Delta^{k} f(i) = \Delta^{k-1} f(i+1) - \Delta^{k-1} f(i)$$

### Application

For the Möbius sequence:
- $\Delta^0[0] = 200$
- $\Delta^1[0] = -100$
- $\Delta^2[0] = 42$
- ...
- $\Delta^9[0] = 8235$

---

## 10. Command

```bash
swind detect 200,100,42,50,11,59,24,29,33,68
```

**Time**: 1.220726ms

---

## 11. Files Modified

- `s-wind-sim/src/knowledge/theorems.rs`: Added Newton Forward Difference Formula, Finite Difference Theorem, Lagrange Interpolation Theorem
- `s-wind-sim/src/fate.rs`: Implemented `compute_forward_differences`, `binomial_coefficient`, `newton_forward_interpolation`, `compute_polynomial_coefficients`

---

*S-wind Formal Algebraic Exploration System v1.1.1*  
*Newton Forward Difference Solver*  
*Author: Jouly / Rogue Intelligence*
