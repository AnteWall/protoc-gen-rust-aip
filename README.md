# protoc-gen-rust-aip

[![CI](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/ci.yml/badge.svg)](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/ci.yml)
[![Release](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/release.yml/badge.svg)](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/AnteWall/protoc-gen-rust-aip/branch/main/graph/badge.svg)](https://codecov.io/gh/AnteWall/protoc-gen-rust-aip)
[![Go Report Card](https://goreportcard.com/badge/github.com/AnteWall/protoc-gen-rust-aip)](https://goreportcard.com/report/github.com/AnteWall/protoc-gen-rust-aip)

A protoc plugin for generating idiomatic Rust code for Google [API Improvement Proposals (AIP)](https://google.aip.dev/) resource names.

> **Note**: All generated resource types use the `ResourceName` suffix for clarity (e.g., `BookResourceName`, `ShelfResourceName`).

## Features

- **Resource Name Generation**: Generates Rust structs and enums for AIP resource names with proper validation, parsing, and formatting
- **Multi-Pattern Support**: Handles resources with multiple patterns using Rust enums for type safety
- **Future Multi-Pattern**: Supports `FUTURE_MULTI_PATTERN` resources for forward compatibility
- **Idiomatic Rust**: Generates code following Rust best practices with proper error handling and trait implementations
- **Comprehensive Validation**: Includes field validation, wildcard detection, and pattern matching
- **Clear Naming**: All generated types use the `ResourceName` suffix (e.g., `BookResourceName`, `ShelfResourceName`) for clarity

## Resource Name Patterns Supported

### Single Pattern Resources
Simple resources with one naming pattern:
```proto
message Book {
  option (google.api.resource) = {
    type: "library.googleapis.com/Book"
    pattern: "projects/{project}/books/{book}"
    singular: "book"
    plural: "books"
  };
}
```

Generates:
```rust
pub struct BookResourceName {
    pub project: String,
    pub book: String,
}
```

### Multi-Pattern Resources
Resources with multiple naming patterns:
```proto
message Shelf {
  option (google.api.resource) = {
    type: "library.googleapis.com/Shelf"
    pattern: "projects/{project}/shelves/{shelf}"
    pattern: "users/{user}/shelves/{shelf}"
    singular: "shelf"
    plural: "shelves"
  };
}
```

Generates:
```rust
pub enum ShelfResourceName {
    Projects(ProjectsShelfResourceName),
    Users(UsersShelfResourceName),
}
```

### Future Multi-Pattern Resources
Resources prepared for future pattern expansion:
```proto
message Author {
  option (google.api.resource) = {
    type: "library.googleapis.com/Author"
    pattern: "authors/{author}"
    history: FUTURE_MULTI_PATTERN
    singular: "author"
    plural: "authors"
  };
}
```

### Nested Resources
Resources nested under other resources:
```proto
message Review {
  option (google.api.resource) = {
    type: "library.googleapis.com/Review"
    pattern: "projects/{project}/books/{book}/reviews/{review}"
    singular: "review"
    plural: "reviews"
  };
}
```

## Installation

### Prerequisites
- Go 1.21+
- Protocol Buffers compiler (`protoc`)
- [Buf CLI](https://buf.build/) (recommended)

### Install the Plugin

```bash
go install github.com/AnteWall/protoc-gen-rust-aip/cmd/protoc-gen-rust-aip@latest
```

## Usage

### With Buf (Recommended)

1. Create a `buf.gen.yaml` file:
```yaml
version: v2
plugins:
  - local: protoc-gen-rust-aip
    out: src/gen
    strategy: directory
```

2. Add dependencies to your `buf.yaml`:
```yaml
version: v2
deps:
  - buf.build/googleapis/googleapis
```

3. Generate code:
```bash
buf generate
```

### With protoc

```bash
protoc --rust-aip_out=./src/gen your_proto_files.proto
```

## Generated Code Features

### Constructors
```rust
let book = BookResourceName::new("my-project", "rust-guide");
```

### Display and Parsing
```rust
// Display
println!("{}", book); // "projects/my-project/books/rust-guide"

// Parse from string
let parsed = BookResourceName::from_str("projects/example/books/test")?;
```

### Validation
```rust
book.validate()?; // Validates field constraints
```

### Wildcard Detection
```rust
let wildcard_book = BookResourceName::new("project", "-");
assert!(wildcard_book.contains_wildcard());
```

### Multi-Pattern Parsing
```rust
// Automatically determines the correct pattern
let shelf = parse_shelf_resource_name("users/alice/shelves/favorites")?;
match shelf {
    ShelfResourceName::Projects(inner) => println!("Project shelf: {}", inner),
    ShelfResourceName::Users(inner) => println!("User shelf: {}", inner),
}
```

## Examples

See the [comprehensive example](examples/) which demonstrates all supported resource name patterns:

```bash
cd examples
cargo run
```

## Architecture

The plugin follows a modular design:

- **Plugin Core** (`internal/genaip/`): Main code generation logic
- **Command Entry Point** (`cmd/protoc-gen-rust-aip/`): Minimal entry point for the protoc plugin
- **Resource Utilities** (`pkg/resourcename/`): Resource name pattern parsing and validation utilities
- **Code Generation**: Generates idiomatic Rust with proper error handling and trait implementations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

[MIT License](LICENSE)

## Related Projects

- [Google AIPs](https://google.aip.dev/) - API Improvement Proposals
- [Buf](https://buf.build/) - Protocol buffer tooling
