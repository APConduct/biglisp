# BigLisp Makefile
# Convenient commands for building and running the BigLisp CLI

.PHONY: help build build-release test clean run-cli repl examples check install

# Default target
help:
	@echo "BigLisp Build Commands:"
	@echo "  make build         - Build all packages in debug mode"
	@echo "  make build-release - Build all packages in release mode"
	@echo "  make test          - Run all tests"
	@echo "  make clean         - Clean build artifacts"
	@echo ""
	@echo "CLI Commands:"
	@echo "  make run-cli       - Run CLI with no arguments (starts REPL)"
	@echo "  make repl          - Start interactive REPL"
	@echo "  make examples      - Show syntax examples"
	@echo "  make check FILE=<file> - Check syntax of a file"
	@echo "  make run FILE=<file>   - Execute a BigLisp file"
	@echo ""
	@echo "Installation:"
	@echo "  make install       - Install CLI binary to ~/.cargo/bin"
	@echo ""
	@echo "Example usage:"
	@echo "  make run FILE=examples/arithmetic.lisp"
	@echo "  make check FILE=examples/comprehensive_demo.lisp"

# Build commands
build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test

clean:
	cargo clean

# CLI commands
run-cli:
	cargo run -p biglisp-cli

repl:
	cargo run -p biglisp-cli -- repl

examples:
	cargo run -p biglisp-cli -- examples

# File operations (use with FILE=path/to/file.lisp)
check:
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make check FILE=examples/arithmetic.lisp"; \
		exit 1; \
	fi
	cargo run -p biglisp-cli -- check $(FILE)

run:
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make run FILE=examples/arithmetic.lisp"; \
		exit 1; \
	fi
	cargo run -p biglisp-cli -- run $(FILE)

# Installation
install:
	cargo install --path biglisp-cli

# Development helpers
dev-check:
	cargo check
	cargo clippy -- -W clippy::all

format:
	cargo fmt

doc:
	cargo doc --open

# Quick demo targets
demo-arithmetic:
	cargo run -p biglisp-cli -- run examples/arithmetic.lisp

demo-control:
	cargo run -p biglisp-cli -- run examples/control_flow.lisp

demo-data:
	cargo run -p biglisp-cli -- run examples/data_structures.lisp

demo-comprehensive:
	cargo run -p biglisp-cli -- run examples/comprehensive_demo.lisp

# Test the actual macro functionality
test-macros:
	cargo test -p biglisp

# All demos in sequence
demo-all: demo-arithmetic demo-control demo-data demo-comprehensive

# Build and run release version
cli-release:
	cargo build --release -p biglisp-cli
	./target/release/biglisp-cli
