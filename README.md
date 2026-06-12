<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/NetScript/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/NetScript?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/NetScript?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
# NetScript

A minimal scripting language lexer and tokenizer, written in Rust.

## Status

| Check | State |
|-------|-------|
| Default branch | `main` |
| CI | ![CI](https://github.com/KooshaPari/NetScript/actions/workflows/ci.yml/badge.svg) |
| License | MIT / Apache-2.0 |

## Architecture

Hexagonal (ports-and-adapters) layout:

```
src/
  domain/      — Token types, lexer logic (pure, no IO)
  ports/       — Trait definitions (input/output contracts)
  adapters/    — CLI adapter, file reader, REPL
  app/         — Composition root (wires adapters to domain)
```

## Quick Start

```sh
# Build
just build

# Run tests (unit + property + snapshot)
just test

# Run the CLI
echo 'let x = 42;' | cargo run

# Lint & format
just lint
```

## Stack

- Rust 2021 edition
- Testing: built-in `#[test]`, `proptest`, `insta`
- CI: GitHub Actions

## License

Dual-licensed under MIT or Apache-2.0 at your option.
