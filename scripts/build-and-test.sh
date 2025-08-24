#!/bin/bash

# Build and test script for protoc-gen-go-aip (Go implementation)

set -e

echo "=== Building protoc-gen-go-aip ==="

# Ensure Go is available
if ! command -v go &> /dev/null; then
    echo "go toolchain not found in PATH. Install Go (https://golang.org/dl/) to build the plugin."
    exit 1
fi

echo "Building Go plugin..."
# Build plugin binary at repo root
go build -o protoc-gen-go-aip main.go
chmod +x protoc-gen-go-aip || true

echo ""
echo "=== Plugin built successfully ==="
echo "Binary location: $(pwd)/protoc-gen-go-aip"
echo ""

echo "Running go tests..."
# Run package tests across the module
go test ./...

# Test with basic example if buf is available
if command -v buf &> /dev/null; then
    echo "=== Testing with basic example ==="
    cd examples/comprehensive

    echo "Generating code with local protoc-gen-go-aip..."
    # Make the plugin discoverable to buf/protoc by adding repo root to PATH
    export PATH="$(pwd)/../..:$PATH"
    buf generate

    echo "Example generation finished."

    cd ../..
else
    echo "buf not found - skipping example test"
    echo "Install buf from https://docs.buf.build/installation to test examples"
fi

echo ""
echo "=== All tests passed! ==="
