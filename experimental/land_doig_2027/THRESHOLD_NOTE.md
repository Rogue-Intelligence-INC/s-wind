# 阈值与“只给最优解”

2026-11-21 00:59（Asia/Shanghai）。产品源码已回滚。`src/bin/swind.rs` 与这次实验开始时一致，工作区无该文件的 diff。

## 原版不改

降低入库阈值的改动只在副本：

`experimental/land_doig_2027/fork/`

它是 `src/bin/swind.rs` 的拷贝。唯一行为差异：`--save-pending` 也接受 `DescriptiveOnly`。编译产物是 `fork/target/release/swind-explore-lowered`，不替换 `target/release/swind`。

对照（序列 `52,45,38`，`--mode verified --save-pending`）：

- 原版：`Not queued: candidate did not survive grading (grade: DescriptiveOnly).`
- 副本：`Queued for human review: discovered_0`，公式仍是 `f(n) = -7·n + 59 [arithmetic]`

三条序列入库后的公式见 `results/lowered_threshold_discovered.json`。每条序列仍只有一个公式。

## 有最优解就不必再给其他

同意。一次 `explore` 的设计就是返回当前最优拟合。候选队列是跨次调用的待审记录，不是同一次搜索里的备选算法。

降低阈值只改变“这个最优拟合能不能写入待审队列”。它不产生第二套策略，也不把拟合公式变成 IIS 删除算法。副本跑出的三条记录仍然是等差式和线性递推式。
