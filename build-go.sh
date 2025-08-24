#!/bin/bash

# Add Go to PATH
export PATH=/usr/local/go/bin:$PATH

# Build the Go protoc plugin
cd go-impl
go mod tidy
go build -o protoc-gen-rust-aip .

# Make it executable
chmod +x protoc-gen-rust-aip

# Copy to a location where protoc can find it
cp protoc-gen-rust-aip /workspaces/protoc-gen-rust-aip/target/debug/

echo "Go implementation built successfully!"
echo "Plugin location: /workspaces/protoc-gen-rust-aip/target/debug/protoc-gen-rust-aip"
