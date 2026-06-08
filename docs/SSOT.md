# SSOT — NetScript

## State
- Default branch: main
- Last verified: 2026-06-08
- CI status: green
- Open PRs: 0
- Open branches: 1 (main)
- Stashes: 0

## Dependencies
- Rust: stable (2021 edition)
- Node: N/A
- Python: N/A

## Architecture
- Hexagonal: yes
- Ports: LexerPort, Tokenizer
- Adapters: CliAdapter
- Domain: Lexer, Token, TokenType

## Next Steps (DAG)
1. [x] P0: State unification
2. [x] P1: Tooling + governance
3. [x] P2: Hexagonal refactor
4. [x] P3: Hardening + SOTA (proptest, insta snapshots)
5. [ ] P4: Expand grammar (parser, AST)
6. [ ] P5: Add REPL adapter

## Fleet Links
- Parent: Phenotype
- Related: Pine (research), melosviz (multi-stack)
- Consumes: N/A
- Merged into: N/A
