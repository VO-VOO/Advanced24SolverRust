# Advanced24Solver

这个仓库现在同时包含两套实现，按语言分目录存放：

- `rust/`: Rust 版本，固定 8 线程并发，支持通过 `A24_THREADS` 覆盖线程数。
- `python/`: Python + Cython 版本，固定 8 进程并发。

两套实现都对应同一问题定义：

- 从 `1..=13` 中抽取 4 个数字，可重复，共 `1820` 种组合。
- 运算支持 `+ - * / ! sqrt x^y log_y(x)`。
- 使用带剪枝的 DFS。
- 结果目标是 `24`，容差 `1e-6`。
- 输出结构化 `results.json`。

## Rust

```bash
cd rust
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run --release
```

## Python

```bash
cd python
uv run python setup.py build_ext --inplace
uv run python main.py
```
