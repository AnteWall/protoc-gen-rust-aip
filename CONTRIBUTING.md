# Contributing to protoc-gen-rust-aip

Thank you for your interest in contributing to protoc-gen-rust-aip! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a new branch for your changes
4. Make your changes
5. Run tests to ensure everything works
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust (latest stable version)
- `protoc` (Protocol Buffers compiler)
- `buf` CLI tool (for testing with examples)

### Building

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build the plugin binary
cargo build --bin protoc-gen-rn
```

### Testing

```bash
# Run unit tests
cargo test --workspace

# Test with examples
cd examples/basic-resource
cargo run --example basic_usage
```

## Project Structure

```
protoc-gen-rust-aip/
├── crates/
│   ├── protoc-gen-rn/       # Main protoc plugin binary
│   ├── resource-codegen/    # Core code generation library
│   └── resource-types/      # Runtime types and traits
├── examples/                # Example proto files and usage
│   ├── basic-resource/      # Simple resource example
│   ├── multi-pattern/       # Multiple patterns example
│   ├── resource-reference/  # Resource references example
│   └── collection-ids/      # Complex nested resources
└── scripts/                 # Build and test scripts
```

## Guidelines

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Add appropriate documentation comments
- Write tests for new functionality

### Commit Messages

- Use clear, descriptive commit messages
- Start with a brief summary (≤50 characters)
- Include detailed explanation if needed

### Pull Requests

- Include tests for new features
- Update documentation as needed
- Ensure all CI checks pass
- Reference any related issues

### Adding Examples

When adding new examples:

1. Create a new directory under `examples/`
2. Include `buf.yaml`, `buf.gen.yaml`, and proto files
3. Add a `Cargo.toml` with an example binary
4. Update the examples README

### Extending the Code Generator

The code generation happens in several phases:

1. **Parsing** (`parse_descriptor.rs`): Extract resource annotations from protobuf descriptors
2. **Rendering** (`render.rs`): Generate Rust code from parsed resources
3. **Emission** (`emit_rust.rs`): High-level interface for code generation

When adding new features:

- Add parsing logic for new annotations in `parse_descriptor.rs`
- Add rendering logic for new code patterns in `render.rs`
- Add tests to verify the generated code works correctly

## AIP Compliance

This project aims to be compliant with Google's API Improvement Proposals (AIPs):

- [AIP-122: Resource names](https://google.aip.dev/122)
- [AIP-123: Resource types](https://google.aip.dev/123)
- [AIP-124: Resource references](https://google.aip.dev/124)

Please ensure any changes maintain compliance with these standards.

## Testing Strategy

- **Unit tests**: Test individual functions and components
- **Integration tests**: Test the plugin end-to-end with example protos
- **Example tests**: Verify generated code compiles and works correctly

## Questions?

If you have questions about contributing, please:

1. Check existing issues and discussions
2. Open a new issue for discussion
3. Reach out to maintainers

Thank you for contributing!
