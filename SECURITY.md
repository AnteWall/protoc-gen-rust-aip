# Security Policy

## Supported Versions

We support the following versions of protoc-gen-rust-aip:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| 0.1.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in protoc-gen-rust-aip, please report it responsibly:

1. **Do not** create a public GitHub issue for security vulnerabilities
2. Email the maintainers directly with details of the vulnerability
3. Include steps to reproduce the issue if possible
4. Provide any relevant code examples or proto files (sanitized)

### What to expect

- Acknowledgment of your report within 48 hours
- Assessment of the vulnerability within 1 week
- Fix development and testing
- Coordinated disclosure timeline discussion
- Credit in the security advisory (if desired)

### Security considerations

This tool generates code from protocol buffer definitions. While the generated code should be safe, potential security concerns include:

- Code injection through malicious proto files
- Path traversal in generated file names
- Memory safety issues in the Go plugin code
- Unsafe code patterns in generated Rust code

Thank you for helping keep protoc-gen-rust-aip secure!
