# Advent of Code 2025

Rust solutions for [Advent of Code 2025](https://adventofcode.com/2025).

## Project Structure

```
aoc-2025/
├── src/
│   ├── main.rs              # Entry point
│   └── bin/
│       └── dayXX/
│           ├── main.rs      # Day solution
│           └── input.txt    # Puzzle input (co-located)
├── docs/
│   └── features/            # Feature documentation
├── devbox.json              # Dev environment config
├── justfile                 # Task runner commands
├── rust-toolchain.toml      # Rust stable channel
└── tarpaulin.toml           # Coverage config (100% threshold)
```

## Development Environment

Uses [devbox](https://www.jetify.com/devbox) with:
- rustup (Rust toolchain manager)
- direnv (automatic environment loading)
- bacon (background Rust code checker)
- just (command runner)

## Commands

```sh
just dev dayXX           # Run a specific day
just watch dayXX         # Watch mode with bacon
just check               # Run all quality gates
just test                # Run tests with coverage
just lint                # Format and lint
```

## Quality Gates

1. `cargo build` - compilation
2. `cargo clippy -- -D warnings` - lints
3. `cargo fmt --check` - formatting
4. `cargo tarpaulin` - tests with 100% coverage

## Progress

| Day | Part 1 | Part 2 |
|-----|--------|--------|
| 01  | 1105   | 6599   |
| 02  | 56660955519 | 79183223243 |
| 03  | 16812  | 166345822896410 |
| 04  | 1549   | 8887   |
| 05  | 694    | 352716206375547 |
| 06  | 6100348226985 | 12377473011151 |
| 07  | 1690   | 221371496188107 |
| 08  | 115885 | 274150525 |
| 09  | 4737096935 | 1644094530 |
| 10  | 422    | 16361  |
| 11  | 470    | 384151614084875 |
| 12  | 595    | N/A    |
