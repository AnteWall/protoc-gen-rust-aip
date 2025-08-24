#!/bin/bash

# Build and test script for protoc-gen-rust-aip

set -e

echo "=== Building protoc-gen-rust-aip ==="

# Build all crates
echo "Building workspace..."
cargo build --workspace

echo "Running tests..."
cargo test --workspace

echo "Building protoc plugin..."
cargo build --bin protoc-gen-rn

echo ""
echo "=== Plugin built successfully ==="
echo "Binary location: target/debug/protoc-gen-rn"
echo ""

# Test with basic example if buf is available
if command -v buf &> /dev/null; then
    echo "=== Testing with basic example ==="
    cd examples/comprehensive
    
    echo "Generating code with local protoc-gen-rn..."
    buf generate

    echo "Running example..."
    cargo run --example comprehensive_usage
    
    cd ../..
else
    echo "buf not found - skipping example test"
    echo "Install buf from https://docs.buf.build/installation to test examples"
fi

echo ""
echo "=== All tests passed! ==="
