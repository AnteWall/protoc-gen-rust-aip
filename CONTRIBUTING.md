# Contributing to protoc-gen-rust-aip

Thank you for your interest in contributing to protoc-gen-rust-aip! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Go 1.21 or later
- Rust 1.70 or later (for testing generated code)
- Protocol Buffers compiler (`protoc`)
- Buf CLI (recommended)

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/protoc-gen-rust-aip.git
   cd protoc-gen-rust-aip
   ```

2. Install dependencies:
   ```bash
   go mod download
   ```

3. Run tests:
   ```bash
   make test
   ```

4. Build the plugin:
   ```bash
   make build
   ```

## Development Workflow

### Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Add or update tests as needed
5. Run the test suite: `make test`
6. Run linting: `make lint`
7. Commit your changes with a descriptive message
8. Push to your fork and create a pull request

### Code Style

- Follow standard Go formatting (`gofmt`)
- Use meaningful variable and function names
- Add comments for exported functions and complex logic
- Keep functions focused and reasonably sized

### Testing

- Add unit tests for new functionality
- Update integration tests if needed
- Test generated Rust code compiles and works correctly
- Run `make test-coverage` to ensure adequate coverage

## Types of Contributions

### Bug Reports

Please use the bug report template and include:
- Steps to reproduce the issue
- Expected vs actual behavior
- Proto files that demonstrate the issue
- Generated code output (if applicable)

### Feature Requests

Please use the feature request template and include:
- Use case description
- Example proto files
- Expected generated code
- Alternatives considered

### Code Contributions

We welcome:
- Bug fixes
- New features
- Performance improvements
- Documentation improvements
- Test improvements

## Code Generation Guidelines

When modifying code generation:

1. **Generated code quality**: Ensure generated Rust code is:
   - Idiomatic and follows Rust conventions
   - Clippy-clean (no warnings)
   - Well-documented with proper rustdoc comments
   - Efficient and minimal overhead

2. **Backward compatibility**: Maintain compatibility with existing proto files unless making a breaking change (which requires version bump)

3. **Error handling**: Provide clear error messages for invalid proto configurations

4. **Testing**: Test with various proto file configurations:
   - Simple resources
   - Multiple patterns
   - Child resources
   - Edge cases

## Pull Request Process

1. Ensure all tests pass
2. Update documentation if needed
3. Add entry to CHANGELOG.md
4. Ensure your PR has a clear description
5. Link any related issues
6. Request review from maintainers

### PR Requirements

- [ ] Tests pass
- [ ] Linting passes
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Generated code examples provided (if applicable)

## Release Process

Releases are handled by maintainers:

1. Update version in relevant files
2. Update CHANGELOG.md
3. Create release PR
4. Tag release after merge
5. GitHub Actions handles building and publishing

## Questions?

- Create a discussion for general questions
- Create an issue for bugs or feature requests
- Check existing issues and discussions first

Thank you for contributing!
