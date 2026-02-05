# Exhaustive Search for Aerodynamically Optimal Aircraft Configurations:
## A First-Principles Framework with Semi-Empirical Correlations

**S-wind Research Team**

*January 2026*

---

## Abstract

We present a systematic approach to identifying aerodynamically optimal subsonic aircraft configurations through exhaustive parameter space search. Our methodology combines first-principles aerodynamic equations with carefully documented semi-empirical correlations, explicitly distinguishing between physically grounded relationships and heuristic surrogate models. We evaluate over 10 million design candidates across a 5-dimensional parameter space comprising aspect ratio, sweep angle, taper ratio, thickness-to-chord ratio, and camber. Unlike previous optimization studies, we introduce explicit structural penalties and Reynolds-number-dependent friction coefficients, resulting in more physically realistic optimal configurations. Our best design achieves a lift-to-drag ratio of 27.74 at an aspect ratio of 18.3, with all solutions satisfying declared constraints—addressing a common self-consistency issue in parametric aircraft optimization literature. We provide complete algorithmic descriptions and numerical verification chains to ensure reproducibility.

**Keywords:** Aircraft design optimization, exhaustive search, aerodynamic efficiency, lift-to-drag ratio, semi-empirical modeling

---

## 1. Introduction

### 1.1 Motivation

The quest for aerodynamically efficient aircraft configurations has driven aerospace engineering for over a century. While modern computational fluid dynamics (CFD) and multi-disciplinary optimization (MDO) frameworks offer sophisticated approaches to this problem, there remains value in understanding the fundamental trade-offs through systematic parameter space exploration.

Many published optimization studies suffer from a subtle but significant issue: *constraint-solution inconsistency*. Authors declare constraints (e.g., Oswald efficiency e > 0.5) but report optimal solutions that violate these constraints (e.g., e = 0.46). This undermines the validity of the optimization claims and creates confusion for practitioners attempting to apply reported results.

### 1.2 Contributions

This paper makes the following contributions:

1. **Self-consistent optimization framework**: We provide two constraint modes with guaranteed solution consistency, explicitly verified for each result.

2. **Transparent model hierarchy**: We clearly distinguish between:
   - First-principles relationships (lift/drag equations, induced drag theory)
   - Semi-empirical correlations (Oswald efficiency, friction coefficient)
   - Heuristic surrogates (camber drag bucket model, structural penalty)

3. **Physical realism improvements**: We incorporate Reynolds-number-dependent friction coefficients and aspect-ratio-dependent structural penalties, addressing the tendency of simplified models to predict unrealistically high aspect ratios.

4. **Complete reproducibility**: All equations, algorithms, and numerical verification chains are provided to enable independent verification.

---

## 2. Theoretical Framework

### 2.1 First-Principles Aerodynamics

The fundamental aerodynamic force equations follow from dimensional analysis and momentum conservation:

$$L = \frac{1}{2}\rho V^2 S C_L$$

$$D = \frac{1}{2}\rho V^2 S C_D$$

where ρ is air density, V is freestream velocity, S is reference wing area, and C_L, C_D are lift and drag coefficients respectively.

From Prandtl's lifting-line theory [Prandtl 1918], the drag coefficient decomposes as:

$$C_D = C_{D0} + \frac{C_L^2}{\pi e \cdot AR}$$

where C_{D0} is the zero-lift drag coefficient, e is the Oswald span efficiency factor, and AR = b²/S is the aspect ratio.

### 2.2 Maximum Lift-to-Drag Ratio

The lift-to-drag ratio is:

$$\frac{L}{D} = \frac{C_L}{C_D} = \frac{C_L}{C_{D0} + K C_L^2}$$

where K = 1/(πe·AR) is the induced drag factor.

Differentiating with respect to C_L and setting equal to zero:

$$\frac{d(L/D)}{dC_L} = \frac{C_{D0} - K C_L^2}{(C_{D0} + K C_L^2)^2} = 0$$

yields the optimal lift coefficient:

$$C_{L,opt} = \sqrt{\frac{C_{D0}}{K}} = \sqrt{C_{D0} \cdot \pi e \cdot AR}$$

Substituting back, the maximum lift-to-drag ratio is:

$$\boxed{\left(\frac{L}{D}\right)_{max} = \frac{1}{2}\sqrt{\frac{\pi e \cdot AR}{C_{D0}}}}$$

This derivation is exact within the parabolic drag polar assumption.

---

## 3. Semi-Empirical Correlations

### 3.1 Oswald Span Efficiency Factor

We employ the widely-used correlation from Raymer (1992):

$$e_{base} = 1.78(1 - 0.045 \cdot AR^{0.68}) - 0.64$$

with corrections for sweep angle Λ and taper ratio λ:

$$e = e_{base} \cdot \cos^{0.5}(\Lambda) \cdot (1 - 0.25|\lambda - 0.4|)$$

**Applicability:** 4 < AR < 40, M < 0.6, conventional planforms.

### 3.2 Skin Friction Coefficient

Following Schlichting (1979), the turbulent flat-plate friction coefficient is:

$$C_f = \frac{0.455}{(\log_{10} Re)^{2.58}}$$

where the Reynolds number is computed as:

$$Re = \frac{V \cdot \bar{c}}{\nu}$$

**Applicability:** Re > 5 × 10⁵ (fully turbulent).

### 3.3 Form Factor

The wing form factor from Hoerner (1965):

$$FF = 1 + 1.2 \cdot (t/c) + 100 \cdot (t/c)^4$$

**Applicability:** t/c < 20%, streamlined airfoils.

---

## 4. Heuristic Surrogate Models

> **⚠️ Important Disclaimer:** The following models are heuristic approximations introduced to capture real physical phenomena in a computationally tractable manner. They are *not* first-principles relationships and should be treated as surrogates with limited generalizability.

### 4.1 Camber Drag Bucket Model

Airfoil drag polars exhibit a "bucket" of minimum drag centered around a design lift coefficient. We model this as:

$$\Delta C_{D0,camber} = k_{bucket} \cdot (C_{L,opt} - C_{L,design})^2$$

where:
- k_bucket ≈ 0.005 (empirical coefficient)
- C_{L,design} ≈ 10 × (f/c) (design lift coefficient)
- f/c is the camber ratio

**Justification:** The relationship C_{L,design} ≈ 10 × camber is an engineering approximation. Actual values vary with airfoil family, Reynolds number, and thickness.

**Effect:** This model makes camber an *active* optimization variable—without it, camber would not affect the objective function.

### 4.2 Structural Penalty Model

High aspect ratio wings incur structural weight penalties. We model this as:

$$\Delta C_{D0,struct} = 0.0005 \cdot \left[\left(\frac{AR}{AR_{ref}}\right)^{1.5} - 1\right], \quad AR > AR_{ref}$$

where AR_ref = 10.

**Justification:** Wing structural weight scales approximately as AR^1.5 to AR^2. This penalty prevents the optimizer from selecting arbitrarily high aspect ratios.

---

## 5. Optimization Methodology

### 5.1 Design Variables

| Variable | Symbol | Range | Step | Points |
|----------|--------|-------|------|--------|
| Aspect ratio | AR | 5.0–25.0 | 0.1 | 201 |
| Sweep angle | Λ | 0°–45° | 1° | 46 |
| Taper ratio | λ | 0.20–0.80 | 0.05 | 13 |
| Thickness ratio | t/c | 6%–15% | 1% | 10 |
| Camber ratio | f/c | 0%–8% | 1% | 9 |

**Total search space:** 201 × 46 × 13 × 10 × 9 = **10,817,820** configurations.

### 5.2 Constraint Modes

**Mode A (Strict):** For conventional transport aircraft
- e > 0.5 (reasonable span efficiency)
- AR ≤ 25 (structural feasibility)
- C_{L,opt} < 1.5 (stall margin)

**Mode B (Relaxed):** For high-altitude long-endurance (HALE) UAVs
- e > 0.3 (allows highly tapered wings)
- AR ≤ 40 (advanced composite structures)
- C_{L,opt} < 1.5

### 5.3 Flight Conditions

| Parameter | Value | Source |
|-----------|-------|--------|
| Altitude | 3,000 m | Design choice |
| Velocity | 50 m/s | Cruise condition |
| Air density ρ | 0.909 kg/m³ | ISA atmosphere |
| Kinematic viscosity ν | 1.86 × 10⁻⁵ m²/s | ISA atmosphere |
| Reference wingspan | 30 m | Scaling reference |

### 5.4 Algorithm

```
Algorithm: EXHAUSTIVE_AERODYNAMIC_OPTIMIZATION

Input: Parameter ranges, constraints, flight conditions
Output: Optimal design satisfying all constraints

1. Initialize best_L/D ← 0, best_design ← ∅
2. FOR each (AR, Λ, λ, t/c, f/c) in search space:
3.     Compute e using Oswald correlation
4.     Compute Re from flight conditions
5.     Compute C_f using Schlichting formula
6.     Compute FF using Hoerner formula
7.     Compute C_D0 = C_f·FF·(S_wet/S) + ΔC_D0,camber + ΔC_D0,struct
8.     Compute K = 1/(πe·AR)
9.     Compute C_L,opt = √(C_D0/K)
10.    Compute (L/D)_max using closed-form expression
11.    IF constraints satisfied AND (L/D)_max > best_L/D:
12.        best_L/D ← (L/D)_max
13.        best_design ← current configuration
14. RETURN best_design
```

---

## 6. Results

### 6.1 Optimal Configurations

| Parameter | Mode A (Strict) | Mode B (Relaxed) |
|-----------|-----------------|------------------|
| **Constraints** | | |
| e constraint | > 0.5 | > 0.3 |
| AR constraint | ≤ 25 | ≤ 40 |
| **Optimal Geometry** | | |
| Aspect ratio AR | 18.3 | 18.3 |
| Sweep angle Λ | 0° | 0° |
| Taper ratio λ | 0.40 | 0.40 |
| Thickness t/c | 6% | 6% |
| Camber f/c | 6% | 6% |
| **Aerodynamic Performance** | | |
| **(L/D)_max** | **27.74** | **27.74** |
| Oswald efficiency e | 0.5618 | 0.5618 |
| Optimal C_L | 0.582 | 0.582 |
| Zero-lift drag C_D0 | 0.01049 | 0.01049 |
| **Constraint Verification** | | |
| e > e_min? | ✅ (0.56 > 0.5) | ✅ (0.56 > 0.3) |
| AR ≤ AR_max? | ✅ (18.3 ≤ 25) | ✅ (18.3 ≤ 40) |

**Key observation:** Both constraint modes yield identical optimal solutions, indicating that the structural penalty (not the e constraint) is the binding factor limiting aspect ratio.

### 6.2 Drag Breakdown

| Component | C_D0 Contribution | Percentage |
|-----------|-------------------|------------|
| Skin friction | 0.00722 | 68.8% |
| Structural penalty | 0.00074 | 7.0% |
| Form drag | 0.00053 | 5.0% |
| Fuselage | 0.00200 | 19.1% |
| Camber penalty | 0.00001 | 0.1% |
| **Total** | **0.01049** | 100% |

The negligible camber penalty (0.1%) indicates that the optimal camber (6%) closely matches the design lift coefficient required for minimum drag operation.

### 6.3 Numerical Verification

$$K = \frac{1}{\pi \times 0.5618 \times 18.3} = 0.03096$$

$$C_{L,opt} = \sqrt{\frac{0.01049}{0.03096}} = 0.582$$

$$\left(\frac{L}{D}\right)_{max} = \frac{0.582}{2 \times 0.01049} = 27.74 \quad \checkmark$$

---

## 7. Discussion

### 7.1 Comparison with Literature

| Aircraft/Design | L/D | AR | Sweep |
|-----------------|-----|-----|-------|
| **This work (optimal)** | **27.7** | **18.3** | **0°** |
| Competition sailplane | ~60 | ~40 | 0° |
| Albatross (biological) | ~20 | ~18 | 0° |
| Boeing 787 | ~20 | ~9 | 32° |
| F-22 Raptor | ~8 | ~2.4 | 42° |

Our optimal design falls between high-performance sailplanes and the natural albatross, which is physically reasonable for a 30m-wingspan aircraft optimized for subsonic cruise efficiency.

### 7.2 Why Not Higher Aspect Ratio?

The structural penalty model limits AR to 18.3 rather than the constraint maximum of 25. This suggests that:

1. The drag reduction from increasing AR is offset by structural penalties
2. Real aircraft designs face similar trade-offs
3. Competition sailplanes achieve AR ≈ 40 through advanced composite structures and reduced penalty coefficients

### 7.3 Limitations

1. **Low-fidelity aerodynamics:** The parabolic drag polar and semi-empirical correlations are approximations. CFD or wind tunnel validation would be required for detailed design.

2. **Heuristic surrogates:** The camber drag bucket and structural penalty models are simplified. More accurate versions would require airfoil database queries and structural FEM analysis.

3. **Single-point optimization:** We optimize for a single cruise condition. Multi-point optimization considering takeoff, climb, and loiter would yield different results.

4. **No aeroelastic effects:** Wing flexibility, flutter, and divergence constraints are not modeled.

### 7.4 Addressing Self-Consistency

A key contribution of this work is explicit constraint verification. Many optimization studies implicitly assume constraints are satisfied without verification. We demonstrate that:

- Declaring e > 0.5 while reporting e = 0.46 is a self-consistency error
- Proper constraint handling may yield lower (L/D)_max but more reliable results
- Both constraint modes should be reported when targeting different applications

---

## 8. Conclusion

We presented an exhaustive search framework for subsonic aircraft aerodynamic optimization that addresses common methodological issues in the literature:

1. **Self-consistency:** All reported solutions satisfy declared constraints, verified explicitly.

2. **Model transparency:** First-principles, semi-empirical, and heuristic components are clearly distinguished.

3. **Physical realism:** Reynolds-dependent friction and structural penalties prevent unrealistic optima.

4. **Reproducibility:** Complete algorithmic and numerical details enable independent verification.

The optimal configuration achieves (L/D)_max = 27.74 at AR = 18.3 with zero sweep, 6% thickness, and 6% camber. This represents a physically realistic design point for a subsonic aircraft balancing aerodynamic efficiency against structural weight.

Future work should incorporate higher-fidelity aerodynamic models (XFOIL, panel methods, or CFD), explicit structural weight estimation, and multi-point optimization for realistic mission profiles.

---

## References

1. Abbott, I. H. & Von Doenhoff, A. E. (1959). *Theory of Wing Sections*. Dover Publications.

2. Hoerner, S. F. (1965). *Fluid-Dynamic Drag*. Hoerner Fluid Dynamics.

3. Prandtl, L. (1918). Tragflügeltheorie. I. Mitteilung. *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen*, 451–477.

4. Raymer, D. P. (1992). *Aircraft Design: A Conceptual Approach*. AIAA Education Series.

5. Schlichting, H. (1979). *Boundary Layer Theory*. 7th ed. McGraw-Hill.

---

## Appendix A: ISA Atmosphere Model

The International Standard Atmosphere (ISA) for the troposphere (h < 11,000 m):

$$T = T_0 - L \cdot h$$

$$\rho = \rho_0 \left(\frac{T}{T_0}\right)^{g/(RL) - 1}$$

where T₀ = 288.15 K, ρ₀ = 1.225 kg/m³, L = 0.0065 K/m, g = 9.80665 m/s², and R = 287.058 J/(kg·K).

Dynamic viscosity via Sutherland's formula:

$$\mu = \mu_0 \left(\frac{T}{T_0}\right)^{1.5} \frac{T_0 + S}{T + S}$$

where μ₀ = 1.789 × 10⁻⁵ Pa·s and S = 110.4 K.

## Appendix B: Search Space Verification

Search space cardinality:

$$N = 201 \times 46 \times 13 \times 10 \times 9 = 10,817,820$$

Constraint satisfaction rate: 68.2% (7,381,350 valid configurations).

---

*Report generated by S-wind v3.0 Exact Algebraic Simulation System*

*Verification status: ✅ All constraints satisfied, numerical verification complete*
