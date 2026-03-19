# Advanced24Solver

使用 `uv` 初始化、`Cython` 加速、`ProcessPoolExecutor(max_workers=8)` 并行求解 24 点高级规则版本。

## 初始化命令

```bash
cd ~/Desktop
mkdir -p Advanced24Solver
uv init --package Advanced24Solver
cd Advanced24Solver
uv add cython
```

## 项目文件

- `main.py`: 入口，负责扩展预编译、1820 组数字生成、8 进程并发、JSON 落盘与总耗时统计。
- `solver.pyx`: Cython DFS 求解器，包含浮点容差、严格剪枝、阶乘/开方/乘方/对数规则。
- `setup.py`: `build_ext --inplace` 编译脚本。

## 运行方式

先编译扩展：

```bash
uv run python setup.py build_ext --inplace
```

再执行主程序：

```bash
uv run python main.py
```

执行结束后，会在项目根目录生成 `results.json`。

`results.json` 中的每条记录都包含：

- `cards`: 原始 4 张牌数组。
- `cards_key`: 逗号拼接后的稳定键。
- `solved`: 是否找到解。
- `expression`: 解表达式，若未找到则为 `无解`。
