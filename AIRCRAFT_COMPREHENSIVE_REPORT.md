# 🛩️ S-wind 完整飞行器设计报告

**生成时间**: 2026-01-29
**知识库**: Blondle Aerospace Module

---

## 设计对比总览

| 设计类型 | L/D | AR | 后掠角 | 厚度 | 马赫数 |
|----------|-----|-----|--------|------|--------|
| 亚音速高效 (M < 0.6) | 31.2 | 23.0 | 0° | 6.0% | 0.40 |
| 高空长航时 UAV (M ≈ 0.5) | 31.2 | 23.0 | 0° | 6.0% | 0.50 |
| 高亚音速运输 (M ≈ 0.85) | 24.7 | 12.0 | 20° | 8.0% | 0.85 |
| 超音速巡航 (M ≈ 1.8) | 5.1 | 5.0 | 40° | 3.0% | 1.80 |

---

## 使用的物理定律

### 升力方程

$$L = \frac{1}{2}\rho V^2 S C_L$$

气动升力计算

### 阻力方程

$$D = \frac{1}{2}\rho V^2 S C_D$$

气动阻力计算

### 抛物线阻力极曲线

$$C_D = C_{D0} + \frac{C_L^2}{\pi e AR}$$

阻力系数与升力系数关系

### Oswald 效率因子

$$e \approx 1.78(1-0.045AR^{0.68})-0.64$$

翼展效率估算

### 最大升阻比

$$(L/D)_{max} = \frac{1}{2}\sqrt{\frac{\pi e AR}{C_{D0}}}$$

理论最大升阻比

### Breguet 航程公式

$$R = \frac{V}{c}\frac{L}{D}\ln\frac{W_1}{W_2}$$

巡航航程计算

### 激波阻力

$$C_{D,wave} \propto (M-M_{crit})^2$$

跨音速/超音速激波阻力

---

## 3D 模型文件

- `optimal_aircraft_subsonic_efficient.obj` - 亚音速高效 (M < 0.6)
- `optimal_aircraft_subsonic_efficient.stl` - 3D打印格式
- `optimal_aircraft_hale_uav.obj` - 高空长航时 UAV (M ≈ 0.5)
- `optimal_aircraft_hale_uav.stl` - 3D打印格式
- `optimal_aircraft_high_subsonic.obj` - 高亚音速运输 (M ≈ 0.85)
- `optimal_aircraft_high_subsonic.stl` - 3D打印格式
- `optimal_aircraft_supersonic.obj` - 超音速巡航 (M ≈ 1.8)
- `optimal_aircraft_supersonic.stl` - 3D打印格式
