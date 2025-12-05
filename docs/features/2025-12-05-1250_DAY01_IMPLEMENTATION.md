# Day 01 Implementation

## Summary

Implemented Advent of Code 2025 Day 1 solution with full test coverage and established project conventions for future days.

## What Was Implemented

### Day 1 Puzzle Solution

**Part 1**: Count how many times a dial lands on 0 after rotations
- Dial has numbers 0-99 (circular)
- Starts at position 50
- L/R rotations with wrap-around using `rem_euclid(100)`

**Part 2**: Count every time the dial passes through 0 during any rotation
- Mathematical approach: calculate crossings using modular arithmetic
- `count_zeros(position, distance, is_left)` helper function

### Project Structure

```
src/
├── main.rs              # Entry point (prints greeting)
└── bin/
    └── day01/
        ├── main.rs      # Day 1 solution
        └── input.txt    # Puzzle input (co-located)
```

**Decision**: Co-locate input files with day code instead of separate `inputs/` folder
- Cleaner organization
- Each day is self-contained
- `include_str!("input.txt")` works directly

### Testing Setup

- **Framework**: Built-in Rust test framework with `#[cfg(test)]` modules
- **Coverage**: cargo-tarpaulin with 100% minimum threshold
- **Test types**:
  - Example input tests
  - Edge case tests (large rotations)
  - Panic tests with `#[should_panic]`
  - Main function execution tests

### Configuration Files

**tarpaulin.toml**:
- `fail-under = 100` (strict coverage)
- `run-types = ["Tests", "Bins"]` (include binary coverage)

**justfile commands**:
- `just dev day01` - one-shot run
- `just watch day01` - watch mode with bacon
- `just test` - run tests with coverage
- `just check` - all quality gates

### Quality Gates

1. `cargo build` - compilation
2. `cargo clippy -- -D warnings` - lints
3. `cargo fmt --check` - formatting
4. `cargo tarpaulin` - tests + 100% coverage

## Answers

- **Part 1**: 1105
- **Part 2**: 6599

## Rust Concepts Learned

- `#[cfg(test)]` - conditional compilation for tests
- `use super::*` - import from parent module
- `println!` macro vs functions
- `rem_euclid()` for proper modulo with negatives
- `include_str!()` for compile-time file embedding
- `#[should_panic]` for testing error paths
