# List available recipes
default:
    @just --list

setup:
    yarn install --immutable

# Collect flow files from react-native
collect:
    cargo run -- collect

# Parse collected flow files
parse:
    cargo run -- parse

# Collect + Parse
run-all: collect parse

# Lint
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format check
fmt-check:
    cargo fmt --all -- --check

# Format
fmt:
    cargo fmt --all

# Run all checks
check: lint fmt-check
