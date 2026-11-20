# 三项小实验（2026-11-21，北京时间）

基准提交 `81b0fa21f9261486252d1321e71163444f4c52c3`。本轮新增代码只在 `experimental/three_projects/`。`src/` 与正式 `swind` 二进制没有改动，降阈值副本没有参与。没有新的 Python 依赖。官方仓库只在 `/tmp` 克隆，用来读题和跑验证器，没有向 SAIR、ca-tools 或 Leaps in Bounds 提交。

环境：Linux，Python 3.12.3，rustc 1.83.0，cargo 1.83.0。S-wind 调用的是仓库里已编译的 `target/release/swind`。

## 覆盖数组

素材：https://chrispbsmolen.ca-tools 页面当时返回 502，改读 https://github.com/chrispbsmolen/ca-tools ，提交 `830809df6946f6d15129ee3863124bff5c3e3f0b`（2026-11-20 16:35 北京时间）。表数据 CC BY 4.0，代码 MIT。Rule 7：表收数组和证书，不收求解代码。本实验没有开 PR。

引擎：没有调用 S-wind。搜索和检查都是 `covering_array/ca_lib.py`。

复现 `CA(4;2,3,2)`。表上的 N 是 4（Kleitman–Spencer / Katona，以及 NIST、KSK 的 VERIFIED 行）。数学下界 `N >= v^t = 4`，Schönheim 下界也是 4。N=3 的有限扫描找不到覆盖数组：3 行放不下 4 个二元对，列对检验在进入内层前就失败。N=4 的有序列三元组里有 48 个通过，内层节点 384，用时约 0.00017 秒。扫描得到的第一例和仓库文件 `CA_4_2_3_2.txt` 不同，两者都通过独立检查器。这是复现，不是新纪录。

复现 `CA(5;2,4,2)`。表上的 N 是 5。`v^t=4` 排除不了 N=4。无序互异列的穷举：N=4 找到 0 个，N=5 找到 80 个，第一例通过检查。这是该参数的有限域穷尽证明，最优值 5，仍然是复现。

改进尝试 `CA(11;2,5,3)`。表上最小 N 是 11，文件 `CA_11_2_5_3_c33e3aff.txt` 经本检查器确认是覆盖数组。Schönheim / `v^2` 下界是 9，所以 N=10 没有被这个下界否定。做法是删掉发布数组的一行，再做单格爬山。种子 20261121，预算 600 秒，实际 600.000 秒，11 轮、16,557,492 次改动，最好覆盖 86/90。没有得到合法的 10 行数组。搜索不完备，不能把失败说成 N=10 不存在，也没有改纪录。

## Andrews–Curtis Discovery

素材：https://competition.sair.foundation/competitions/acc/discovery 。截止 2026-11-30 23:59 AoE，即北京时间 2026-12-01 19:59。验证器仓库 https://github.com/SAIRcompetition/Andrews-Curtis ，提交 `a0fd6e6f52d82c93ccc06ab91d81e8fa3678256e`，Apache-2.0。没有登录，没有报名，没有上传。

引擎：S-wind 未调用。移动语义来自官方 `verifier.core.apply_move`，最终是否接受以官方 `python3 -m verifier` 的退出码和 `ok` 为准。搜索队列是本目录的 BFS。

公开训练解复现：`competition/examples/sample_submission.txt`。验证器返回码 0，`accepted: true`。`ms-train-0160` 长度 7，`ok: true`，work 46。`sac-train-0160` 长度 9，`ok: true`，work 47。训练题不计分，不是比赛胜利。

自己搜的训练题 `ms-train-0001`，初始字 `[[-1,2,1,-2,-2],[1,2]]`。BFS 上限深度 8、字长和 24、节点 2,000,000。1.004 秒、49,420 个展开节点、见过 307,094 个状态，找到 `[6,2,9,5,11]`。官方验证器 `ok: true`，长度 5，work 28，证书 `sha256:8e036890afb4069ae833e5c0984ca13e3bce1b994237f0de3529e3ef8c2affe2`。仍是训练题。

计分题 `ac-08491`（官方 manifest 里 `ac-*` 初始字长和最短的一条），初始字 `[[-2,-1],[-2,-2,-1,-1,2,2,1]]`。上限深度 6、字长和 20。0.616 秒展开 152,767 个节点后队列在这个上限内排空，没有路径。这只说明「深度 ≤ 6 且每步字长和 ≤ 20」里没有解。比赛允许 100,000 步、字长和 10,000、work 5,000,000。所以不能说这题无 AC 序列，也不能算比赛成绩。

## Golomb 尺

页面 https://leapsinbounds.org/constants/golomb-ruler-29/ 在 2026-11-21 01:19（北京时间）为下界 578（Lean 证明）和上界 623（已检查构造）。同站小阶页面给出 G(4)=6 到 G(10)=55。本实验没有重跑他们的 Lean 证明，也没有把 578 当成自己的下界。

引擎：尺的搜索是 `golomb/run_golomb.py`。S-wind 只在枚举之后拟合长度序列，见下。

2 到 10 阶：从鸽笼下界 `n(n-1)/2` 起，按规范形（平移到 0、反射取字典序较小的一侧）找第一把尺。每把尺再经独立检查：正差两两不同。结果与页面对得上，且搜索在找到该长度之前已经排除更短的长度，所以这些阶有穷尽最优证明，状态是复现：

| 阶 | 长度 | 节点 | 秒 | 最优证明 |
| --- | ---: | ---: | ---: | --- |
| 2 | 1 | 0（下界已是 1，直接检验 `[0,1]`） | 0.00 | 有 |
| 3 | 3 | 1 | 0.00 | 有 |
| 4 | 6 | 2 | 0.00 | 有 |
| 5 | 11 | 45 | 0.00 | 有 |
| 6 | 17 | 697 | 0.00 | 有 |
| 7 | 25 | 12,720 | 0.01 | 有 |
| 8 | 34 | 160,476 | 0.17 | 有 |
| 9 | 44 | 1,676,837 | 2.31 | 有 |
| 10 | 55 | 15,791,170 | 25.77 | 有 |

11 阶：鸽笼下界 55。180 秒、94,750,365 个节点，长度 55 到 64 已搜完且没有尺，65 被超时打断。因此本轮只证明 G(11) ≥ 65。没有找到尺，没有证明页面上的 72，也没有最优证明。

29 阶：自己的下界只有鸽笼 406，不是 578。确定性贪心得到长度 1158，检查通过。种子 20261121 的随机贪心跑满 120 秒、60,334 次，最短 1102，检查通过。两者都大于 623。没有改上界，更不是最优。搜索不完备。

S-wind：`swind explore 1,3,6,11,17,25,34,44,55 --mode verified`。返回 Newton 8 次式，`Fit level: InterpolantFit`，等级 `TrivialInterpolation`，DoF=0（9 个点、9 个参数），`Is discovery: false`。这是事后插值，不是 Golomb 搜索，也不证明任何一阶的最优长度。

## 复现

```bash
bash experimental/three_projects/run_all.sh
python3 experimental/three_projects/test_fast.py
```

`run_all.sh` 会重新克隆两个官方仓库并重跑全部搜索，覆盖数组改进尝试固定 600 秒，11 阶 180 秒，29 阶 120 秒。`test_fast.py` 只重跑小阶，几秒内结束。

结果目录：

- `experimental/three_projects/covering_array/results/`
- `experimental/three_projects/andrews_curtis/results/`
- `experimental/three_projects/golomb/results/`
