# S-wind 使用方法审计

审计前 HEAD：`a36cbcbfe5ea4f6fa34ac4e2d5e50ba586cbb2d2`。分支 `cursor/land-doig-experiment-17b2`。`git diff --stat main -- src Cargo.toml` 无输出，产品 `src/` 与 `main` 一致。本次只新增 `experimental/usage_audit/`。没有改 `src/`、产品二进制、阈值，也没有覆盖 Land–Doig 与三项实验的原始日志。

运行时刻：2026-11-21 03:50（北京时间，UTC+8）。机器 UTC 记录为 `2026-11-20T19:50:15Z`。原始输出见 `logs/run.log`。

## 正确调用实际接到哪里

`swind explore` 是 CLI 主入口，分发在 `src/bin/swind.rs:158-178`。

| 模式 | 调用 |
| --- | --- |
| `fast` | `Fate::explore`。源码打印说明该路径标签未分级 |
| `verified` | `explore_with_verification` |
| `full` | `full_exploration`，并打印 Voyager 方程 |
| 默认，以及其余字符串 | `explore_with_self_loop` |

未给 `--kb` 时，`src/bin/swind.rs:142-154` 使用 `KnowledgeBase::initialize_fundamentals()`。`--save-pending` 只接受 `is_discovery` 或 `CandidateLaw|StrongCandidate|GenuineDiscovery`（`src/bin/swind.rs:199-203`）。

这条链做数列识别、内置序列对照、拟合分级和自循环变换。`src/fate.rs:2774-2791` 规定：信号签名重复才把 `closure_reached` 设为 true；得到非平凡模式或已知序列就停止。本次库调用的正对照是 `closure_reached=false`，`iterations=1`。

CLI 没有接到的库搜索是 `AlgebraicSearchEngine`。`search_linear` 在 `src/search.rs:215-255`，对每个基和整数系数 `a,b`（`b≠0`）构造 `a+b·base`，按容差留下结果。`search_quadratic` 从 `src/search.rs:258` 开始，内层范围写死 `-100..=100`（第 260 行），`with_coeff_range` 不进入这个循环。`is_exact` 的定义是 `error_percent < tolerance`（`src/symbolic/algebraic.rs:573-576`）。数值来自 `AlgebraicBase::to_f64` 的 `f64`（`src/symbolic/algebraic.rs:55-71`），不是 EPR。

`swind optimize`（`src/bin/swind.rs:262-301`）不调用 `batch_optimizer`。A1 的搜索空间是字面量 `673_920`。它用 LCG 抽 `sqrt(N)` 个点，目标是 6 个 `k/100` 的平均值，并且只在 `objective > 0.0` 时更新“最优”。钢材公式在 `src/bin/batch_optimizer.rs:676-694`，采样在 `737-777`，只属于另一个二进制。本次没有运行那个二进制。

`src/bin/industrial_demos.rs` 没有引用 `s_wind`。

`GravitationalField::gravitational_force`（`src/quad.rs:89-96`）计算固定公式 `F=GM/r²`。默认 `G=1`、`M=10`（`src/quad.rs:35-64`）。它不搜索参数。

## 本次跑过的命令和数字

产品二进制用的是已有 `/workspace/target/release/swind`，本次没有重编。`optimize` 的工作目录是 `/tmp`，避免把 `swind_output/` 写进仓库。

1. `cargo test --lib -- test_fate_linear_epr test_fate_fibonacci_epr test_basel_s20 test_quad_system_integration -- --test-threads=8`  
   4 passed，0 failed，0.00s。包含 `test_basel_s20` 与 `test_quad_system_integration`。

2. `cargo test --lib fate::tests::test_fate_linear_epr -- --exact --nocapture`  
   `formula: "f(n) = 1/2·n [arithmetic]"`，1 passed。

3. `experimental/usage_audit/caller` 的 `cargo run --release`。知识库同样是 `initialize_fundamentals`，信号是 `1/2,1,3/2`。  
   `explore`、`explore_with_verification`、`explore_with_self_loop` 都给出 `f(n) = 1/2·n [arithmetic]`。验证路径 `fit=DescriptiveModel`，`discovery=false`。自循环 `closure_reached=false`，`truly_exhausted=false`，`iterations=1`。

4. CLI：
   - `swind explore "1/2,1,3/2" --mode loop`：同一公式。`DescriptiveOnly`，OEIS 搜索 129 条，`OOS 0/1`，`DoF=2 (n=3, params=1)`，`Is discovery: false`。自循环 1 轮，`Fate: CONVERGED`。
   - `swind explore "1,1,2,3,5" --mode full`：`Known { OEIS: A000045 ... (phi^n - psi^n) / sqrt(5) }`。`Voyager equation: Some("f: unknown (5 values)") (verified: true)`。`Is discovery: false`。
   - `swind explore "22,42,122"`：`s[n+1] = 4·s[n] + -46`，`DescriptiveOnly`，`OOS 0/1`，`DoF=1`。
   - `swind explore "3,6,9,12" --mode verified`：`f(n) = 3·n`，`DescriptiveOnly`，`DoF=3 (n=4, params=1)`，`OOS 0/1`。
   - `swind explore "2,3,5,7,11,13" --mode loop`：`KnownSequence A000040`，1 轮停止。
   - `swind explore "8,3,15,6,20,1" --mode loop`：4 轮、3 次变换，公式 `s[n+2] = 3512/5327·s[n+1] + 63590/143829·s[n]`，仍是 `DescriptiveOnly`，`OOS 0/1`。没有 closure 证书。
   - `swind optimize --project A1`：搜索空间 673920，样本 820，`optimal_value` 0.1483，820/820。JSON 在 `/tmp/swind_output/A1_result.json`。

5. 同一 `search_linear`，只改目标和基：
   - 目标 `1+2π`，基 `Pi`，系数 `-3..=3`，容差 `0.001%`。1 条命中：`1 + 2·π`，误差 0，值 `7.2831853071795862`。
   - 目标 `5+√2`，基 `Sqrt(2)`，系数 `-5..=5`。1 条命中：`5 + √2`，误差 0，值 `6.4142135623730949`。

6. `python3 experimental/usage_audit/independent_check.py` 只重数上述整数盒：
   - `one_plus_two_pi examined=42 hits=1`，`a=1 b=2`，同一浮点值。
   - `five_plus_sqrt2 examined=110 hits=1`，`a=5 b=1`，同一浮点值。
   - `six_zeros_mean=0` 只说明六个 0 的平均值是 0。它没有枚举 `101^6`。

`src/search.rs:464-478` 的 `test_search_alpha_inv` 本次没有运行。它走 `search_quadratic`，内层固定 `-100..=100`，默认 9 个基，规模是 `9 × 201³ = 73085409` 次浮点评估，断言结果非空且最佳误差 `< 0.1%`。小盒线性结果不能代替这次未跑的测试。

## 两组具体证据

有限整数盒：`search_linear` 对声明范围内的 `a+b·base` 逐项算相对误差。42 项和 110 项都由独立脚本逐项重数。两边各留下同一个系数对，打印的 `f64` 相同，误差都是 0。换目标后系数从 `(1,2)` 变成 `(5,1)`，所以输出不是写死的一个数字。证据止于这个浮点容差；`is_exact` 不是有理数恒等。

固定公式：默认 `G=1,M=10`，`r=2` 时库返回 `5/2`，与 `src/lib.rs:196` 的断言一致，且对应单测通过。输入改为 `r=4` 后库返回 `5/8`，等于 `10/16`。这是把新距离代入同一公式。

CLI 的 0.1483 是 820 个样本中的最小正均值。源码在目标大于 0 时才记录，六个 0 不会被记成最优。打印的 673920 也不是 `batch_optimizer` 里 A1 参数网格：按 `calculate_search_space` 的 `(max-min)/step+1`，六维点数是 `4×11×16×13×9×9 = 741312`。CLI 没有调用 `evaluate_building`，所以 0.1483 不是钢材用量。

`full` 模式对 `1,1,2,3,5` 命中的是内置 OEIS 记录。Voyager 行是 `f: unknown (5 values)` 且 `verified: true`。这不是对一个未知优化目标的最优性证明。

## 先前四项调用了哪条路

- Land–Doig：`experimental/land_doig_2027/test_longer_sequence.sh` 与 `RESULTS.md` 的命令是 `swind explore`，模式为默认 loop 或 `--mode verified`，输入是短整数列，部分带 `--save-pending`。没有调用 `AlgebraicSearchEngine`、`swind optimize`、`batch_optimizer` 或 `full`。降阈值副本只改队列门槛。
- 覆盖数组：`experimental/three_projects/covering_array` 中没有 `swind` 调用。搜索在实验目录的 Python 里。
- Andrews–Curtis：`run_ac.py` 记录 `swind_called: false`。
- Golomb：长度来自实验目录的 Python。随后 `run_golomb.py:244` 执行 `swind explore <九个长度> --mode verified`。结果是 Newton 8 次式、`InterpolantFit`、`TrivialInterpolation`、DoF=0、`Is discovery: false`。那次没有进入系数搜索。

`explore` 没有导出这些题目的算法，不能推出整个库没有有限搜索。`search_linear` 能在小盒里找回 `a+b·base`，也不能推出它已经能解覆盖数组、AC 或 Golomb：这些对象没有被写成该引擎的代数基和整数系数。

## 旧结论里哪些撤回、哪些保留

撤回把“S-wind 不是算法产生者，已经证伪”当作全系统结论。当时的运行只覆盖 `explore` 对短数列的拟合。`search_linear` 会在给定整数盒里枚举并返回命中式，而 `explore` 和 `optimize` 都没有调用它。

保留：

- Land–Doig 短列和 Golomb 长度列上，`explore --mode verified` 的输出是拟合分级，不是这些题目的搜索。
- 覆盖数组、AC、Golomb 小阶已经记下的数字仍属于当时的 Python 与官方检查器，本审计不改写。
- `swind optimize --project A1` 的 0.1483 不是全网格最优，也不是钢材公式的值。
- 本次没有出现 `closure_reached=true`，没有 EPR 最优性证书。
- `is_exact` 是浮点百分比。

## 下一步最小闭环

要继续核对“现有库在有限系数盒上找式，并和整盒重数一致”：调用 `AlgebraicSearchEngine::search_linear`，更换目标和基，把盒子保持到可以整盒重数。不新增搜索器，不改 `src`。

要得到 Land–Doig、覆盖数组、AC 或 Golomb 的最优解：现有 `explore` 不接收这些模型。下一步是把题目写成引擎已经会枚举的对象，或继续由题目自己的枚举负责。这是模型是否适配的问题，不是再加一个求解器，也不是再改阈值。`batch_optimizer` 的 A1/A2 是手写估算加采样，本次未跑，不能直接当作这个闭环。

若要把库内二次搜索测试当作正对照，需要单独跑完 `test_search_alpha_inv` 并记下它打印的表达式和误差。本次没有这个输出。

## 未完成

- 未运行 `test_search_alpha_inv` / `search_quadratic`。
- 未运行 `batch_optimizer`，没有 A1 钢材公式的实测值。
- 没有一组输入在本次把 `closure_reached` 置为 true。
- 其他助手的既往调用记录尚未并入。这里不推断那些记录。

复现：

```bash
cd /workspace
cargo test --lib fate::tests::test_fate_linear_epr -- --exact --nocapture
cd experimental/usage_audit/caller && cargo run --release
python3 ../independent_check.py
cd /tmp
/workspace/target/release/swind explore "1/2,1,3/2" --mode loop
/workspace/target/release/swind explore "1,1,2,3,5" --mode full
/workspace/target/release/swind optimize --project A1
```
