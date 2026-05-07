# LottieCode

Lottie as Code —— a compiled DSL for [Lottie](https://lottiefiles.com/) animations.

AI writes `.lc` files, the compiler emits Lottie JSON.

[![CI](https://github.com/AgentsMesh/LottieCode/actions/workflows/ci.yml/badge.svg)](https://github.com/AgentsMesh/LottieCode/actions/workflows/ci.yml)
[![Nightly](https://github.com/AgentsMesh/LottieCode/actions/workflows/nightly.yml/badge.svg)](https://github.com/AgentsMesh/LottieCode/actions/workflows/nightly.yml)

## 为什么

直接让 Agent 写 Lottie JSON 不够好——字段缩写（`gr/tr/sh`）、`{a,k}` 包装、Bezier 切线、Group↔Transform 1:1 强约束榨干认知带宽，没余力做"设计"。

LottieCode 把"写 Lottie 动画"做成"写 Motion DSL → 编译为 Lottie JSON"，让 Agent 在设计层（命名缓动 / 动效预设 / 设计 token / 时序编排）操作，编译器吃掉机械层。

## 快速开始

```bash
git clone https://github.com/AgentsMesh/LottieCode.git
cd LottieCode

# 编译 example
bazel run //crates/lottiecode-cli:lottiecode-cli -- \
    build examples/14-gaming-restore/main.lc -o /tmp/gaming.json

# 反编译既有 Lottie
bazel run //crates/lottiecode-cli:lottiecode-cli -- \
    decompile some.json -o some.lc
```

## CLI 子命令

| 命令 | 用途 |
|---|---|
| `lc check <file>` | 仅验证 DSL 是否合法 |
| `lc build <file> [-o out]` | 编译为 Lottie JSON / dotLottie |
| `lc plan <file>` | 树形打印 IR 结构 |
| `lc inspect <file> [--json]` | 检视编译产物 |
| `lc fmt <file> [-w]` | 格式化 DSL |
| `lc syntax` | 输出完整 DSL 语法参考（供 LLM 上下文） |
| `lc decompile <file> [-o out]` | Lottie JSON → DSL（实验性） |

## 架构

5 个 crate：

- **lottiecode-lang** — DSL → AST → IR（lexer / parser / semantic）
- **lottiecode-motion** — 命名 easing → cubic Bezier 表
- **lottiecode-codegen** — IR → Lottie JSON（保持 AE 字段顺序）
- **lottiecode-decompile** — Lottie JSON → DSL（含反 AE 模式检测）
- **lottiecode-cli** — clap 子命令入口

## 工程纪律

- 单文件 ≤ 200 行
- 中文注释
- 字段顺序、enum 类型、AE 默认值省略策略均在 codegen 层契约化
- 13 个 example 配 `expected.json` 作为回归基线
- Round-trip 测试：JSON → DSL → JSON 结构等价比对

## 安装预编译二进制

每天自动构建 nightly：

```bash
# Linux x86_64
curl -L https://github.com/AgentsMesh/LottieCode/releases/download/nightly/lottiecode-linux-x86_64.tar.gz | tar xz

# macOS Apple Silicon
curl -L https://github.com/AgentsMesh/LottieCode/releases/download/nightly/lottiecode-macos-arm64.tar.gz | tar xz

# macOS Intel
curl -L https://github.com/AgentsMesh/LottieCode/releases/download/nightly/lottiecode-macos-x86_64.tar.gz | tar xz
```

## 姐妹项目

- [VEAC](https://github.com/AgentsMesh/veac) — Video Editing as Code
- [Pastel](https://github.com/AgentsMesh/pastel) — Design as Code

## License

MIT
