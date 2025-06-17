# masked-email-cli project commands

# Default recipe to run when just is called without arguments
default:
    @just --list

# Clean the project
clean:
    cargo clean

# Format code using rustfmt
format:
    cargo fmt --all

# Run linting checks with clippy
lint:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Build the project in release mode
build:
    cargo build --release

# Run the project in debug mode
run:
    cargo run
