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
