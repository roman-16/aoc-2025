# Project Initialization

## Overview
Initialize the Advent of Code 2025 Rust project with a basic project structure and a simple main.rs file.

## Requirements
- Create a standard Rust project structure
- Include a normal main.rs file (simple hello world or minimal entry point)
- Set up according to AGENTS.md architecture guidelines

## Plan

### Steps
1. Initialize Rust project with `cargo init`
2. Create `rustfmt.toml` with default settings
3. Create `rust-toolchain.toml` with stable channel
4. Create `devbox.json` with: rustup, direnv, bacon, just
5. Create `.envrc` for direnv integration
6. Create `justfile` with common commands
7. Create `src/bin/` directory for daily solutions
8. Verify project compiles and runs (quality gates)

### File Structure (After Implementation)
```
aoc-2025/
├── .envrc
├── AGENTS.md
├── Cargo.toml
├── devbox.json
├── justfile
├── rust-toolchain.toml
├── rustfmt.toml
├── docs/
│   └── features/
│       └── 2025-12-04-2311_PROJECT_INITIALIZATION.md
├── src/
│   ├── bin/
│   └── main.rs
└── .opencode/
```

## Q&A
- **Toolchain channel?** → stable
- **Include rustfmt.toml?** → yes
- **Include rust-toolchain.toml?** → yes
- **Dev environment?** → devbox with rustup, direnv, bacon, just

## Status
- [x] Completed

## Quality Gates
- [x] `cargo build` - passed
- [x] `cargo clippy -- -D warnings` - passed
- [x] `cargo fmt --check` - passed
- [x] `cargo test` - passed (0 tests)
