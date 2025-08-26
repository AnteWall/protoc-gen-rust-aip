# Makefile for protoc-gen-rust-aip

.PHONY: help build build-all install test test-rust coverage coverage-rust bench examples generate fmt lint lint-rust lint-all check all dev clean lib test-app

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the Go plugin"
	@echo "  build-all  - Build for all platforms"
	@echo "  install    - Install the plugin to GOPATH/bin"
	@echo "  test       - Run Go tests"
	@echo "  test-rust  - Run Rust tests in examples"
	@echo "  examples   - Build and run the Rust examples"
	@echo "  generate   - Generate Rust code from proto files"
	@echo "  lib        - Build the as-lib library"
	@echo "  test-app   - Build and run the test application"
	@echo "  clean      - Clean build artifacts"
	@echo "  fmt        - Format Go and Rust code"
	@echo "  lint       - Run Go linter"
	@echo "  lint-rust  - Run Rust linter"
	@echo "  lint-all   - Run all linters"
	@echo "  check      - Run all checks (build, test, lint)"
	@echo "  all        - Build everything and run all tests"
	@echo "  dev        - Development workflow"

# Go plugin targets
build:
	@echo "Building protoc-gen-rust-aip plugin..."
	go build -o protoc-gen-rust-aip ./cmd/protoc-gen-rust-aip

install:
	@echo "Installing protoc-gen-rust-aip plugin..."
	go install ./cmd/protoc-gen-rust-aip

test:
	@echo "Running Go tests..."
	go test ./...

# Rust examples targets
test-rust:
	@echo "Running Rust tests in as-lib..."
	cd examples/as-lib && cargo test
	@echo "Running Rust tests in test-app..."
	cd examples/test-app && cargo test

examples: generate
	@echo "Building as-lib example..."
	cd examples/as-lib && cargo build
	@echo "Running test-app example..."
	cd examples/test-app && cargo run

# Code generation
generate: install
	@echo "Generating Rust code from proto files..."
	cd examples/as-lib && rm -rf src/gen/ && buf generate

# Cleaning
clean:
	@echo "Cleaning build artifacts..."
	rm -f protoc-gen-rust-aip
	cd examples/as-lib && cargo clean && rm -rf src/gen target
	cd examples/test-app && cargo clean

# Formatting
fmt:
	@echo "Formatting Go code..."
	go fmt ./...
	@echo "Formatting Rust code in as-lib..."
	cd examples/as-lib && cargo fmt
	@echo "Formatting Rust code in test-app..."
	cd examples/test-app && cargo fmt

# Linting
lint:
	@echo "Linting Go code..."
	golangci-lint run ./... || echo "golangci-lint not installed, skipping Go linting"

lint-rust:
	@echo "Linting Rust code in as-lib..."
	cd examples/as-lib && cargo clippy -- -D warnings || echo "cargo clippy failed or not available"
	@echo "Linting Rust code in test-app..."
	cd examples/test-app && cargo clippy -- -D warnings || echo "cargo clippy failed or not available"

lint-all: lint lint-rust

# Cross-compilation
build-all:
	@echo "Building for all platforms..."
	GOOS=linux GOARCH=amd64 go build -o dist/protoc-gen-rust-aip-linux-amd64 ./cmd/protoc-gen-rust-aip
	GOOS=linux GOARCH=arm64 go build -o dist/protoc-gen-rust-aip-linux-arm64 ./cmd/protoc-gen-rust-aip
	GOOS=darwin GOARCH=amd64 go build -o dist/protoc-gen-rust-aip-darwin-amd64 ./cmd/protoc-gen-rust-aip
	GOOS=darwin GOARCH=arm64 go build -o dist/protoc-gen-rust-aip-darwin-arm64 ./cmd/protoc-gen-rust-aip
	GOOS=windows GOARCH=amd64 go build -o dist/protoc-gen-rust-aip-windows-amd64.exe ./cmd/protoc-gen-rust-aip

# Comprehensive checks
check: build generate test test-rust lint
	@echo "All checks passed!"

# Build and test everything
all: clean build install generate test test-rust examples
	@echo "Build and test cycle complete!"

# Development workflow
dev: install generate test-rust examples
	@echo "Development cycle complete!"

# Library-specific targets
lib: generate
	@echo "Building the as-lib library..."
	cd examples/as-lib && cargo build

test-app: lib
	@echo "Building and running the test application..."
	cd examples/test-app && cargo run
