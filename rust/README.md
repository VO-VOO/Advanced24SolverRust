# Advanced24SolverRust

Rust 版本的高级 24 点求解器，功能与桌面上的 Python/Cython 项目对应：

- 数字范围 `1..=13`，四张牌，可重复，共 `1820` 种组合。
- 运算支持 `+ - * / ! sqrt x^y log_y(x)`。
- 使用带剪枝的 DFS。
- 浮点判定容差 `1e-6`。
- 限制负数开方、非法对数、过大阶乘和一元运算嵌套。
- 固定 `8` 线程并发。
- 输出结构化 `results.json`。

## 检查

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## 运行

```bash
cargo run --release
```
