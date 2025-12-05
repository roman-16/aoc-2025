# Advent of Code 2025

Rust solutions for [Advent of Code 2025](https://adventofcode.com/2025).

## Project Structure

```
aoc-2025/
├── src/
│   ├── main.rs          # Main entry point
│   └── bin/             # Daily solutions (day01.rs, day02.rs, etc.)
├── docs/
│   └── features/        # Feature documentation
├── devbox.json          # Dev environment config
├── justfile             # Task runner commands
├── rust-toolchain.toml  # Rust stable channel
└── rustfmt.toml         # Formatter config
```

## Development Environment

Uses [devbox](https://www.jetify.com/devbox) with:
- rustup (Rust toolchain manager)
- direnv (automatic environment loading)
- bacon (background Rust code checker)
- just (command runner)

## Commands

```sh
cargo build              # Build project
cargo run --bin dayXX    # Run specific day
cargo test               # Run tests
cargo clippy             # Lint
cargo fmt                # Format code
```

## Status

Project initialized and ready for puzzle solutions.
