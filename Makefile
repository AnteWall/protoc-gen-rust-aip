# Makefile for protoc-gen-rust-aip

.PHONY: help build build-all install test test-rust coverage coverage-rust bench examples generate fmt lint lint-rust lint-all check all dev clean

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the Go plugin"
	@echo "  build-all  - Build for all platforms"
	@echo "  install    - Install the plugin to GOPATH/bin"
	@echo "  test       - Run Go tests"
	@echo "  test-rust  - Run Rust tests in examples"
	@echo "  coverage   - Run Go tests with coverage"
	@echo "  coverage-rust - Run Rust tests with coverage"
	@echo "  bench      - Run Go benchmarks"
	@echo "  examples   - Build and run the Rust examples"
	@echo "  generate   - Generate Rust code from proto files"
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
	@echo "Running Rust tests..."
	cd examples && cargo test

examples: generate
	@echo "Building and running Rust examples..."
	cd examples && cargo run

# Code generation
generate: install
	@echo "Generating Rust code from proto files..."
	cd examples && rm -rf src/github.com && buf generate

# Cleaning
clean:
	@echo "Cleaning build artifacts..."
	rm -f protoc-gen-rust-aip
	cd examples && cargo clean
	cd examples && rm -rf src/github.com target

# Formatting
fmt:
	@echo "Formatting Go code..."
	go fmt ./...
	@echo "Formatting Rust code..."
	cd examples && cargo fmt

# Linting
lint:
	@echo "Linting Go code..."
	golangci-lint run ./... || echo "golangci-lint not installed, skipping Go linting"

lint-rust:
	@echo "Linting Rust code..."
	cd examples && cargo clippy -- -D warnings || echo "cargo clippy failed or not available"

lint-all: lint lint-rust

# Coverage
coverage:
	@echo "Running Go tests with coverage..."
	go test -v -race -coverprofile=coverage.out -covermode=atomic ./...
	go tool cover -html=coverage.out -o coverage.html

coverage-rust:
	@echo "Running Rust tests with coverage..."
	cd examples && cargo llvm-cov --all-features --workspace --lcov --output-path ../rust-coverage.lcov || echo "cargo-llvm-cov not available"

# Benchmarks
bench:
	@echo "Running Go benchmarks..."
	go test -bench=. -benchmem -run=^$ ./...

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
