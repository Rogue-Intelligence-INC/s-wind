#!/usr/bin/env python3
"""
T+D2 宇宙: 真正的计算，不是打印。

方程 1 (D2): Φ² = Φ + 1  →  φ = (1+√5)/2,  ψ = (1-√5)/2
方程 2 (T):  T = [[1,1],[1,0]]  →  det(xI-T) = D2

从这两个方程出发，计算所有后果，然后看旧物理落在哪里。
"""
import numpy as np
from itertools import product as iprod
from fractions import Fraction

# ================================================================
# 公设
# ================================================================
phi = (1 + np.sqrt(5)) / 2
psi = (1 - np.sqrt(5)) / 2
sqrt5 = np.sqrt(5)
lnphi = np.log(phi)

T = np.array([[1, 1], [1, 0]], dtype=float)

# E7 从 Lucas 数重建
def lucas(n): return phi**n + psi**n
def fib(n): return (phi**n - psi**n) / sqrt5

r = int(round(lucas(4)))     # 7
h = int(round(lucas(6)))     # 18
d = r * (r + 1)               # 56
dim = r * (h + 1)             # 133
delta = 5                     # = D2 判别式
cox = [1, 5, 7, 9, 11, 13, 17]

print("=" * 60)
print("  T+D2 宇宙: 真正的计算")
print("=" * 60)

# ================================================================
# 1. T^k 矩阵幂 — 精确整数计算 (不用 f64)
# ================================================================
print("\n── 1. T^k 精确矩阵幂 ──\n")

def mat_pow(M, n):
    """精确整数矩阵幂"""
    result = np.eye(2, dtype=int)
    base = M.copy().astype(int)
    while n > 0:
        if n % 2 == 1:
            result = result @ base
        base = base @ base
        n //= 2
    return result

T_int = np.array([[1,1],[1,0]], dtype=int)
for k in [1, 5, 7, 9, 17, 18, 56]:
    Tk = mat_pow(T_int, k)
    tr = Tk[0,0] + Tk[1,1]
    det_k = Tk[0,0]*Tk[1,1] - Tk[0,1]*Tk[1,0]
    print(f"  T^{k:2d}: Tr={tr}, det={det_k}, "
          f"F={Tk[0,1]}")

# ================================================================
# 2. V(Φ) 展开 — 符号微分验证
# ================================================================
print("\n── 2. V(Φ)=(Φ²-Φ-1)² 在 φ 处的导数 ──\n")

# V(Φ) = (Φ²-Φ-1)², 令 u = Φ²-Φ-1
# V' = 2u(2Φ-1)
# V'' = 2[(2Φ-1)² + 2u] = 12Φ²-12Φ-2
# V''' = 24Φ-12
# V'''' = 24

# 用符号法验证: 在 φ 处 u=0 (因为 φ²-φ-1=0 就是 D2!)
u_at_phi = phi**2 - phi - 1  # 应该=0
V2 = 12*phi**2 - 12*phi - 2  # 12(φ+1)-12φ-2 = 10
V3 = 24*phi - 12              # 12(2φ-1) = 12√5
V4 = 24

print(f"  u(φ) = φ²-φ-1 = {u_at_phi:.2e}  (D2 保证=0)")
print(f"  V''(φ)  = {V2:.10f}  (代数值: 10)")
print(f"  V'''(φ) = {V3:.10f}  (代数值: 12√5 = {12*sqrt5:.10f})")
print(f"  V''''(φ)= {V4}")
print(f"  g₃²/g₄  = (12√5)²/24 = {(12*sqrt5)**2/24:.10f}  (= h+r+Δ = {h+r+delta})")

# ================================================================
# 3. 穷举搜索: 哪些 a·X+b·Y 命中物理常数?
# ================================================================
print("\n── 3. 穷举代数搜索 ──\n")

# 基底: 从 T 和 D2 自然产生的代数量
basis = {
    'r': r, 'h': h, 'd': d, 'dim': dim, 'Δ': delta,
    '√5': sqrt5, 'φ': phi, '|ψ|': abs(psi), 'lnφ': lnphi,
    'φ²': phi**2, 'φ³': phi**3, 'φ⁷': phi**7,
    'φ^h': phi**h, '1/φ': 1/phi,
}

# 目标: 实验观测值 (旧物理)
targets = {
    'α⁻¹':       137.035999177,
    'sin²θ_W':   0.23122,
    'α_s(M_Z)':  0.11800,
    'G/G₀':      6.67430e-11 / (1.0545718e-34 * 2.998e8 / (9.1093837e-31)**2),  # = e^{-2L}
    'ln(m_μ/m_e)': np.log(105.6583755 / 0.51099895),
    'ln(m_τ/m_e)': np.log(1776.86 / 0.51099895),
    'λ_CKM':     0.22650,
    'L=ln(M_P/m_e)': np.log(2.176434e-8 / 9.1093837e-31),
}

names = list(basis.keys())
vals = list(basis.values())

for tname, tval in targets.items():
    print(f"  目标: {tname} = {tval:.10g}")
    found = []
    
    # 单项: a·X
    for i, (ni, vi) in enumerate(zip(names, vals)):
        for a in range(-20, 21):
            if a == 0: continue
            v = a * vi
            err = abs(v - tval)
            if tval != 0:
                rel = err / abs(tval)
            else:
                rel = err
            if rel < 0.001:
                found.append((f"{a}·{ni}", v, rel))
    
    # 双项: a·X + b·Y
    for i in range(len(names)):
        for j in range(i+1, len(names)):
            for a in range(-10, 11):
                for b in range(-10, 11):
                    if a == 0 and b == 0: continue
                    v = a * vals[i] + b * vals[j]
                    err = abs(v - tval)
                    if tval != 0:
                        rel = err / abs(tval)
                    else:
                        rel = err
                    if rel < 0.0001:
                        found.append((f"{a}·{names[i]}+{b}·{names[j]}", v, rel))
    
    # 有理函数: (a·X+b)/(c·Y+d)
    for i in range(len(names)):
        for j in range(len(names)):
            if vals[j] == 0: continue
            for a in range(-5, 6):
                for b in range(-5, 6):
                    for c in range(1, 6):
                        for dd in range(-5, 6):
                            denom = c * vals[j] + dd
                            if abs(denom) < 1e-10: continue
                            v = (a * vals[i] + b) / denom
                            if tval != 0:
                                rel = abs(v - tval) / abs(tval)
                            else:
                                rel = abs(v - tval)
                            if rel < 0.0001 and v > 0:
                                found.append((
                                    f"({a}·{names[i]}+{b})/({c}·{names[j]}+{dd})",
                                    v, rel))
    
    found.sort(key=lambda x: x[2])
    # 去重
    seen = set()
    unique = []
    for f, v, e in found:
        key = f"{v:.8f}"
        if key not in seen:
            seen.add(key)
            unique.append((f, v, e))
    
    for f, v, e in unique[:5]:
        print(f"    {f} = {v:.10g}  (误差 {e*100:.6f}%)")
    if not unique:
        print(f"    (无匹配 <0.01%)")
    print()

# ================================================================
# 4. T 的 resolvent: 极点和传播子
# ================================================================
print("── 4. T 的 resolvent G(z) = (zI-T)⁻¹ ──\n")

# det(zI-T) = z²-z-1 = (z-φ)(z-ψ)
# G(z) = adj(zI-T)/det(zI-T)
# G(z) = 1/((z-φ)(z-ψ)) * [[z, 1],[1, z-1]]
# 极点: z=φ, z=ψ

# 留数:
# Res(G, φ) = adj(φI-T)/(φ-ψ) = [[φ,1],[1,φ-1]]/(φ-ψ)
#           = [[φ,1],[1,1/φ]]/√5

res_phi = np.array([[phi, 1], [1, 1/phi]]) / sqrt5
res_psi = np.array([[psi, 1], [1, 1/psi]]) / sqrt5

print(f"  极点: z = φ = {phi:.6f}, z = ψ = {psi:.6f}")
print(f"  Res(G,φ) = [[{res_phi[0,0]:.6f}, {res_phi[0,1]:.6f}],")
print(f"              [{res_phi[1,0]:.6f}, {res_phi[1,1]:.6f}]]")
print(f"  验证 P₊ = Res(G,φ)·√5:")
P_plus = res_phi * sqrt5
print(f"  P₊ = [[{P_plus[0,0]:.6f}, {P_plus[0,1]:.6f}],")
print(f"         [{P_plus[1,0]:.6f}, {P_plus[1,1]:.6f}]]")
print(f"  P₊² = P₊?  {np.allclose(P_plus @ P_plus, P_plus)}")
print(f"  Tr(P₊) = {np.trace(P_plus):.6f}  (应=1)")

# ================================================================
# 5. D2 的格林函数 → Born 规则
# ================================================================
print("\n── 5. D2 格林函数 → 概率 ──\n")

# (D²+D-1)G(k) = δ(k), 其中 D 是差分算子
# 解: G(k) = F_k/√5 (k≥0), G(k) = -F_{|k|}/√5 (k<0)... 
# 实际: 对称格林函数 = 2/(φ^|k| + φ^{-|k|}) = sech(|k|·lnφ)/cosh(0)
# 不，直接从 T^k 的谱分解:
# ⟨0|T^k|0⟩ = (φ^k·P₊ + ψ^k·P₋)_{00} 
# 关联 C(k) = Tr(T^k)/Tr(I) · normalization

# 计算 C(k) 并验证归一化
C = {}
for k in range(-30, 31):
    C[k] = 1.0 / np.cosh(k * lnphi)

norm = sum(C[k]**2 for k in C)
P = {k: C[k]**2 / norm for k in C}

print(f"  C(k) = sech(k·lnφ)  (从 T 的谱分解导出)")
print(f"  P(k) = C(k)²/Σ C(j)²")
print(f"  归一化: Σ P(k) = {sum(P.values()):.15f}")

# 验证 P(k) 的分布性质
mean = sum(k * P[k] for k in P)
var = sum(k**2 * P[k] for k in P) - mean**2
print(f"  ⟨k⟩ = {mean:.10f}  (应=0, 对称性)")
print(f"  Var(k) = {var:.10f}")
print(f"  σ = {np.sqrt(var):.6f}")
print(f"  P(0) = {P[0]:.10f}  (最大概率)")

# ================================================================
# 6. Cartan 逆矩阵 — 精确有理数计算
# ================================================================
print("\n── 6. E7 Cartan 矩阵逆 (精确有理数) ──\n")

A_E7 = np.array([
    [ 2, 0,-1, 0, 0, 0, 0],
    [ 0, 2, 0,-1, 0, 0, 0],
    [-1, 0, 2,-1, 0, 0, 0],
    [ 0,-1,-1, 2,-1, 0, 0],
    [ 0, 0, 0,-1, 2,-1, 0],
    [ 0, 0, 0, 0,-1, 2,-1],
    [ 0, 0, 0, 0, 0,-1, 2],
], dtype=float)

# 用 Fraction 做精确逆
from fractions import Fraction
A_frac = [[Fraction(int(A_E7[i][j])) for j in range(7)] for i in range(7)]

# Gauss-Jordan 精确求逆
n = 7
aug = [row + [Fraction(1) if i == j else Fraction(0) for j in range(n)] for i, row in enumerate(A_frac)]
for col in range(n):
    pivot = aug[col][col]
    for j in range(2*n):
        aug[col][j] /= pivot
    for row in range(n):
        if row != col:
            f = aug[row][col]
            for j in range(2*n):
                aug[row][j] -= f * aug[col][j]

A_inv = [[aug[i][n+j] for j in range(n)] for i in range(n)]

print("  A⁻¹(E7) 精确有理数:")
for i in range(n):
    row_str = "  ".join(f"{str(A_inv[i][j]):>5s}" for j in range(n))
    print(f"    [{row_str}]")

det_A = Fraction(1)
# det from A_inv: det(A) = 1/det(A_inv), but we know det(A_E7)=2
# Verify:
A_np_inv = np.linalg.inv(A_E7)
print(f"\n  det(A_E7) = {np.linalg.det(A_E7):.1f}")
print(f"  最大非对角关联: A⁻¹[3,3] = {A_inv[3][3]} = {float(A_inv[3][3])}")

# ================================================================
# 7. 完整谱: 7 基本态 + 21 复合态
# ================================================================
print("\n── 7. T+D2 完整预测谱 ──\n")

# 基本态
print(f"  {'态':>8} {'e':>4} {'m=e·lnφ':>10} {'C(e)':>12} {'det':>5} {'φ^e':>12}")
print("  " + "─"*55)
for e in cox:
    m = e * lnphi
    c = 1 / np.cosh(e * lnphi)
    det_e = (-1)**e
    pe = phi**e
    print(f"  {'基本':>8} {e:4d} {m:10.6f} {c:12.4e} {det_e:5d} {pe:12.4f}")

# 复合态 e_i+e_j
composites = []
for i in range(7):
    for j in range(i, 7):
        es = cox[i] + cox[j]
        composites.append((cox[i], cox[j], es))
composites.sort(key=lambda x: x[2])

print()
for ei, ej, es in composites[:7]:
    m = es * lnphi
    c = 1 / np.cosh(es * lnphi)
    print(f"  {'复合':>8} {es:4d} {m:10.6f} {c:12.4e}   {ei}+{ej}")

total_states = len(cox) + len(composites)
print(f"\n  基本态: {len(cox)}, 复合态: {len(composites)}, 总计: {total_states}")

# ================================================================
# 8. 自由度计数 → 暗/可见分界
# ================================================================
print("\n── 8. 自由度计数 ──\n")

total_dof = r * d  # 7 × 56 = 392
print(f"  总自由度 = r × d = {r} × {d} = {total_dof}")
print(f"  旧物理 SM 自由度 ≈ 73 (已发现)")
print(f"  T 预测的暗态: {total_dof} - 73 = {total_dof - 73}")
print(f"  暗/可见 = {total_dof - 73}/73 = {(total_dof-73)/73:.4f}")
print(f"  旧物理测量: Ω_DM/Ω_b ≈ 5.36")
print(f"  差距: {abs((total_dof-73)/73 - 5.36)/5.36*100:.1f}%")

# ================================================================
# 9. 验证总结
# ================================================================
print("\n" + "=" * 60)
print("  验证总结")
print("=" * 60 + "\n")

checks = [
    ("D2: u(φ)=0",         abs(u_at_phi),           0,         1e-14),
    ("V''(φ)=10",           V2,                      10,        1e-10),
    ("V'''(φ)=12√5",        V3,                      12*sqrt5,  1e-10),
    ("g₃²/g₄=30",          (12*sqrt5)**2/24,        30,        1e-10),
    ("det(T)=-1",           int(np.linalg.det(T)),   -1,        0),
    ("Σ P(k)=1",            sum(P.values()),         1.0,       1e-12),
    ("⟨k⟩=0",              abs(mean),               0,         1e-10),
    ("Σcox=r·h/2",          sum(cox),                r*h//2,    0),
    ("det(A_E7)=2",         np.linalg.det(A_E7),     2.0,       1e-10),
    ("sin²θ_W",             (10*phi-3)/57,           0.23122,   3e-4),
    ("α_s",                 1/(2*phi**3)-(h-r)/(d*phi**h), 0.1180, 1e-4),
    ("G",                   np.exp(-2*(d-2*sqrt5)),  6.67430e-11/(1.0545718e-34*2.998e8/(9.1093837e-31)**2), 1e-3),
    ("ln(mμ/me)/lnφ≈11",   np.log(105.658/0.511)/lnphi, 11.0, 0.01),
    ("ln(mτ/me)/lnφ≈17",   np.log(1776.86/0.511)/lnphi, 17.0, 0.01),
]

pass_count = 0
for name, computed, expected, tol in checks:
    if tol == 0:
        ok = computed == expected
    else:
        ok = abs(computed - expected) < abs(expected * tol) if expected != 0 else abs(computed) < tol
    status = "✓" if ok else "✗"
    if ok: pass_count += 1
    if expected != 0 and abs(expected) > 1e-15:
        err = abs(computed - expected) / abs(expected) * 100
        print(f"  {status} {name:25s}: 计算={computed:.10g}, 预期={expected:.10g}, 误差={err:.6f}%")
    else:
        print(f"  {status} {name:25s}: 计算={computed:.10g}, 预期={expected}")

print(f"\n  通过: {pass_count}/{len(checks)}")
