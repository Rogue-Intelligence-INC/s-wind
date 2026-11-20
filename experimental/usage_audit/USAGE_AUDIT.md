# S-wind 使用方法审计

审计前 HEAD：`a36cbcbfe5ea4f6fa34ac4e2d5e50ba586cbb2d2`。分支 `cursor/land-doig-experiment-17b2`。`git diff --stat main -- src Cargo.toml` 无输出，产品 `src/` 与 `main` 一致。本次只新增 `experimental/usage_audit/`。没有改 `src/`、产品二进制、阈值，也没有覆盖 Land–Doig 与三项实验的原始日志。

运行时刻：2026-11-21 03:50（北京时间，UTC+8）。机器 UTC 记录为 `2026-11-20T19:50:15Z`。原始输出见 `logs/run.txt`。根目录 `.gitignore` 忽略 `*.log`，所以记录使用 `.txt`。

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

- 未运行 `test_search_alpha_inv` / `search_quadratic`。本轮没有再加 `search_linear` 小盒。
- 未运行 `batch_optimizer`，没有 A1 钢材公式的实测值。
- 没有一组输入在本次把 `closure_reached` 置为 true。
- Iggy 转述的 Studio 多阶段项目见下一节。远端树里没有那些文件。用户本机没有被访问，所以本机是否另有一份不能从这次核对里得出。

## 工业例子正对照（2026-11-21 03:55 北京时间）

上一节的系数搜索和 `swind optimize --project A1` 不是这条正对照。本轮跑的是远端已经存在的 `examples/industry_beam_optimize.rs`。它由 `c21cac6`（2026-10-30 03:44 北京时间）加入，提交说明写的是 GB 50017 × GB/T 11263、120 个组合、最优 `32663/700`。`origin/main` 就是这个提交。本分支产品历史没有比它更新的 `src/`。

### 远端没有 Iggy 记录里的那套 pipeline

`git ls-tree` 在 `origin/main` 和本分支上都没有 `pipelines/`。`*_optimize.rs` 只有 `examples/industry_beam_optimize.rs`。没有锅炉、冷库、泵管、电池、光储的例子、manifest，也没有“加载全部 KB → 要求 Closure 和 OPTIMUM → 基线 → 敏感性 → verified/selfloop/full/Voyager → fit → KB → 证据/PDF”的运行器。`ARCHITECTURE.md` 把唯一支持的用户入口写成 `swind explore`。`src/symbolic/epr.rs` 最后一次提交是 `836fbff`（2026-02-05 16:52 北京时间）。2026-11-16 的 `epr.rs` 修复不在这两个远端头上。这些是远端缺口，不是对用户本机目录的判断。本机未访问。Iggy 的四项工业结果和光储 1275300 组合仍是待核转述：远端没有实参和日志可重放。

### 实际命令

```bash
cd /workspace
cargo run --release --example industry_beam_optimize --offline
python3 experimental/usage_audit/beam_independent_check.py
cargo run --release --example industry_beam_optimize --offline -- --span 4000 --spacing 2000
```

cargo 1.83.0。完整程序输出在 `logs/industry_beam_run.txt`。`Cargo.toml` 只显式登记了 `knowledge_demo`，上面第一条命令仍然编译并运行了 `industry_beam_optimize`。

程序输出：10 根截面的质量/Wx 自检都通过断言。`Exhaustive search: 120 combinations, 71 feasible`。第 1 名 `HN300x150 Q355 spacing=3500mm`，打印 `46.6614 CNY/m2`，精确值 `EPR { numerator limbs [32663], denominator limbs [700] }`。后四名打印成本 52.50、54.44、55.62、60.00，应力比/挠度比/剪应力比见日志。

`main` 不读参数。带 `--span 4000 --spacing 2000` 的第二次运行仍是 120/71 和 `32663/700`。这个例子没有配置入口，所以没有做成参数变体，也没有改例子或 `src`。

### 调用链里各部分做什么

搜索是例子自己的三重循环：10 根截面 × 2 个钢号 × 6 个间距（`examples/industry_beam_optimize.rs:74-106`）。排序用 EPR 的 `Ord`（同文件 107 行）。`OPTIMUM` 是排完后的 `println`（114-116 行）。库只提供 `s_wind::EPR`（第 8 行）的运算和比较。没有 `Fate`、`explore`、`AlgebraicSearchEngine`、`KnowledgeBase`、Voyager。`knowledge_base/*.json` 存在，这个例子不读它们。约束、目标、截面表都写在例子里。

README 里的 `CLOSURE(E)` 和 `src/fate.rs:2774-2779` 的 `closure_reached` 属于信号变换。这次运行没有生成那份数据，也没有证据文件或 PDF。打印出来的“proven by exhaustion”指的是这 120 项声明空间被循环走完。

### 独立复核

`beam_independent_check.py` 用 `fractions.Fraction` 重数同一张表和同一组公式，不调用库。结果 `examined=120 feasible=71`，最优 `32663/700`，与 EPR 分子分母一致。前五名的 `sigma/f`、`defl/lim`、`tau/fv` 与程序打印的三位小数一致，精确成本依次为 `32663/700`、`105/2`、`32663/600`、`445/8`、`60/1`。这次核对通过的是这个声明空间上的最小成本，不是 Iggy 记录里的关卡成功。`ARCHITECTURE.md` 仍记载 `divmod` 在 `[2^127, 2^128)` 有已知错误；本例的成本分数远小于该区间，独立分数复核与 EPR 打印一致。

### 和只跑 explore 的差别

这条例子不调用 `swind explore`。先前 Land–Doig、Golomb 长度走的是数列拟合；这个例子走的是手写目录穷举加 EPR 比较。两条路的输出不能互相代替。它也不是 Iggy 描述的多阶段 pipeline：KB 没有参加约束或目标，Closure 没有生成，verified/selfloop/full/Voyager 没有运行。就远端现有的这个例子而言，声明空间被穷尽，最优分数被独立复核通过。Studio 那条闭环在这个远端上没有入口可跑。

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

# 工业例子。A1 不是这条正对照。
cd /workspace
cargo run --release --example industry_beam_optimize --offline
python3 experimental/usage_audit/beam_independent_check.py
```
