# swind-explore-lowered

`src/main.rs` 是 `src/bin/swind.rs` 的副本，不是产品入口。

与原版只差一处：`--save-pending` 也接受 `DescriptiveOnly`。原版只把 `CandidateLaw`、`StrongCandidate`、`GenuineDiscovery` 写入待审队列。

引擎每次仍只给出一个最优拟合。降低阈值只决定这个拟合是否入库，不另造候选，也不把序列拟合变成 IIS 算法。

```bash
cargo build --release --manifest-path experimental/land_doig_2027/fork/Cargo.toml
```

产物：`experimental/land_doig_2027/fork/target/release/swind-explore-lowered`

产品二进制仍是仓库根目录的 `target/release/swind`，由未改动的 `src/bin/swind.rs` 编译。
