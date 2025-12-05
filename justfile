# Display available tasks
default:
    @just --list

# Build the project
build:
    cargo build --release

# Run all quality gates
check: build lint test

# Run a binary (e.g., just dev day01)
dev bin="aoc-2025":
    cargo run --bin {{bin}}

# Watch and re-run on changes (e.g., just watch day01)
watch bin="aoc-2025":
    bacon run -- --bin {{bin}}

# Format code and run clippy
lint:
    cargo fmt
    cargo clippy -- -D warnings

# Run all tests with coverage
test:
    cargo tarpaulin
