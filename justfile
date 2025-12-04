default:
    @just --list

# Build the project
build:
    cargo build

# Build for release
release:
    cargo build --release

# Run the main binary
run:
    cargo run

# Run a specific day (e.g., just day 01)
day DAY:
    cargo run --bin day{{DAY}}

# Run all tests with coverage
test:
    cargo tarpaulin

# Run clippy and check formatting
lint:
    cargo clippy -- -D warnings
    cargo fmt --check

# Run all quality gates
check: build lint test

# Watch for changes with bacon
watch:
    bacon run -- -q
