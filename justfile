# Display available tasks
default:
    @just --list

# Build the project
build:
    cargo build --release

# Run all quality gates
check: build lint test

# Watch for changes with bacon
dev:
    bacon run -- -q

# Run clippy and check formatting
lint:
    cargo clippy -- -D warnings
    cargo fmt --check

# Run all tests with coverage
test:
    cargo tarpaulin
