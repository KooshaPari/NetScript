# NetScript Justfile
set shell := ["bash", "-cu"]

# Show available commands
default:
    @just --list

# Build the workspace
build:
    cargo build --workspace

# Run all tests (unit, property, snapshot)
test:
    cargo test --workspace
    cargo test --workspace -- proptest

# Run linting (clippy + fmt check)
lint:
    cargo fmt -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Auto-format code
fmt:
    cargo fmt

# Run cargo-deny audit
audit:
    cargo deny check

# Generate snapshot tests review
snapshot:
    cargo insta review

# CI-like run (build + test + lint)
ci: build test lint audit

# Clean artifacts
clean:
    cargo clean
