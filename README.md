# LottieCode

Lottie as Code — a compiled DSL for [Lottie](https://lottiefiles.com/) animations.

AI writes `.lc` files, the compiler emits Lottie JSON.

[![CI](https://github.com/AgentsMesh/LottieCode/actions/workflows/ci.yml/badge.svg)](https://github.com/AgentsMesh/LottieCode/actions/workflows/ci.yml)
[![Nightly](https://github.com/AgentsMesh/LottieCode/actions/workflows/nightly.yml/badge.svg)](https://github.com/AgentsMesh/LottieCode/actions/workflows/nightly.yml)

[简体中文](./README.zh-CN.md)

## Why

Asking an AI agent to write Lottie JSON directly is wasteful. Cryptic field names (`gr/tr/sh`), the `{a,k}` wrapping, Bezier tangents, and the rigid 1:1 Group↔Transform pairing burn through the agent's attention budget — leaving nothing for actual *design*.

LottieCode reframes "writing Lottie animations" as "writing a Motion DSL → compiling to Lottie JSON". The agent operates at the design layer (named easings, motion presets, design tokens, timing orchestration); the compiler handles the mechanical layer.

## Quick start

```bash
git clone https://github.com/AgentsMesh/LottieCode.git
cd LottieCode

# Compile an example
bazel run //crates/lottiecode-cli:lottiecode-cli -- \
    build examples/14-gaming-restore/main.lc -o /tmp/gaming.json

# Decompile an existing Lottie file
bazel run //crates/lottiecode-cli:lottiecode-cli -- \
    decompile some.json -o some.lc
```

## CLI subcommands

| Command | Purpose |
|---|---|
| `lc check <file>` | Validate DSL without emitting output |
| `lc build <file> [-o out]` | Compile to Lottie JSON / dotLottie |
| `lc plan <file>` | Print IR as a tree |
| `lc inspect <file> [--json]` | Inspect compilation output |
| `lc fmt <file> [-w]` | Format DSL source |
| `lc syntax` | Print full DSL grammar (for LLM context) |
| `lc decompile <file> [-o out]` | Lottie JSON → DSL (experimental) |

## Architecture

Five crates:

- **lottiecode-lang** — DSL → AST → IR (lexer / parser / semantic)
- **lottiecode-motion** — named easings → cubic Bezier table
- **lottiecode-codegen** — IR → Lottie JSON (preserves AE field order)
- **lottiecode-decompile** — Lottie JSON → DSL (with non-AE-pattern detection)
- **lottiecode-cli** — clap-based command entry point

## Engineering discipline

- Single file ≤ 200 lines
- Field order, enum types, and AE-default-omission strategy are contractualized in the codegen layer
- 13 examples ship with `expected.json` as regression baselines
- Round-trip tests: JSON → DSL → JSON structural equivalence

## Pre-built binaries

Nightly builds run automatically:

```bash
# Linux x86_64
curl -L https://github.com/AgentsMesh/LottieCode/releases/download/nightly/lottiecode-linux-x86_64.tar.gz | tar xz

# macOS Apple Silicon (also runs on Intel via Rosetta 2)
curl -L https://github.com/AgentsMesh/LottieCode/releases/download/nightly/lottiecode-macos-arm64.tar.gz | tar xz
```

Tagged releases (`v*.*.*`) trigger a separate workflow that publishes a stable release with platform binaries and auto-generated release notes.

## Sister projects

- [VEAC](https://github.com/AgentsMesh/veac) — Video Editing as Code
- [Pastel](https://github.com/AgentsMesh/pastel) — Design as Code

## License

MIT
