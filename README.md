# protoc-gen-rust-aip

[![CI](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/ci.yml/badge.svg)](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/ci.yml)
[![Release](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/release.yml/badge.svg)](https://github.com/AnteWall/protoc-gen-rust-aip/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/AnteWall/protoc-gen-rust-aip/branch/main/graph/badge.svg)](https://codecov.io/gh/AnteWall/protoc-gen-rust-aip)
[![Go Report Card](https://goreportcard.com/badge/github.com/AnteWall/protoc-gen-rust-aip)](https://goreportcard.com/report/github.com/AnteWall/protoc-gen-rust-aip)

A protoc plugin for generating idiomatic Rust code for Google [API Improvement Proposals (AIP)](https://google.aip.dev/) resource names, designed to work seamlessly with prost/tonic generated protobuf code.

> **Note**: All generated resource types use the `ResourceName` suffix for clarity (e.g., `BookResourceName`, `ShelfResourceName`).



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

## Quick Start

1. **Install the plugin**:
   ```bash
   go install github.com/AnteWall/protoc-gen-rust-aip/cmd/protoc-gen-rust-aip@latest
   ```

2. **Try the example**:
   ```bash
   git clone https://github.com/AnteWall/protoc-gen-rust-aip.git
   cd protoc-gen-rust-aip
   make test-app
   ```

3. **See the integration in action**:
   The test app demonstrates all features including prost/tonic integration, resource name validation, and gRPC client usage.

## Why Use protoc-gen-rust-aip?

- **Type Safety**: Prevents runtime errors by validating resource names at compile time
- **Standards Compliance**: Follows Google AIP standards for resource naming
- **Seamless Integration**: Works with your existing prost/tonic workflow
- **Developer Experience**: Clear error messages and idiomatic Rust APIs
- **Production Ready**: Comprehensive validation and error handling for production services


## Usage

### Complete Integration with Prost and Tonic

The recommended approach is to create a library crate that includes both the prost-generated protobuf code and the AIP resource names. This creates a reusable library that can be consumed by multiple applications.

#### 1. Create Library Structure

```
your-proto-lib/
├── Cargo.toml
├── buf.gen.yaml
├── buf.yaml
├── proto/
│   └── your_service.proto
└── src/
    ├── lib.rs
    └── gen/          # Generated code goes here
        ├── mod.rs -- dont create
        ├── your_service.rs -- dont create
        ├── your_service.tonic.rs -- dont create
        ├── your_service.serde.rs -- dont create
        └── your_service_aip.rs -- dont create
```

#### 2. Configure buf.gen.yaml

```yaml
version: v2
inputs:
  - directory: proto
plugins:
  # Generate prost protobuf code
  - remote: buf.build/community/neoeinstein-prost:v0.2.3
    out: src/gen
    opt:
      - bytes=.
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
      - file_descriptor_set
  
  # Generate serde implementations
  - remote: buf.build/community/neoeinstein-prost-serde:v0.2.3
    out: src/gen
  
  # Generate tonic gRPC code
  - remote: buf.build/community/neoeinstein-tonic:v0.3.0
    out: src/gen
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
  
  # Generate AIP resource names
  - local: protoc-gen-rust-aip
    out: src/gen
    opt:
      - paths=source_relative
    strategy: all
  
  # Generate Cargo.toml features (optional but recommended)
  - protoc_builtin: prost-crate
    out: .
    strategy: all
    opt:
      - include_file=src/gen/mod.rs
      - gen_crate
```

#### 3. Library Cargo.toml

```toml
[package]
name = "your-proto-lib"
version = "0.1.0"
edition = "2021"

[features]
default = ["proto_full"]
# Features generated by prost-crate
proto_full = ["your-service-v1"]
"your-service-v1" = []

[dependencies]
bytes = "1.1.0"
prost = "0.13.1"
pbjson = "0.7"
pbjson-types = "0.7"
serde = "1.0"
tonic = { version = "0.12", features = ["gzip"] }
```

#### 4. Library src/lib.rs

```rust
// Include generated protobuf modules
include!("gen/mod.rs");

// Include AIP resource names
include!("gen/your_service_aip.rs");

// Re-export commonly used types for convenience
pub use your_service::v1::*;
```

#### 5. Generate Code

```bash
buf generate
```

#### 6. Use in Applications

Create applications that depend on your library:

```toml
# Application Cargo.toml
[dependencies]
your-proto-lib = { path = "../your-proto-lib" }
tokio = { version = "1.0", features = ["full"] }
```

```rust
// Application main.rs
use your_proto_lib::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use generated protobuf types
    let request = YourRequest {
        name: "projects/my-project/resources/my-resource".to_string(),
        // ... other fields
    };
    
    // Use AIP resource names with validation
    let resource_name = YourResourceResourceName::new("my-project", "my-resource");
    resource_name.validate()?;
    
    println!("Resource: {}", resource_name);
    
    // Use with gRPC clients
    let mut client = YourServiceClient::connect("http://localhost:8080").await?;
    let response = client.your_method(request).await?;
    
    Ok(())
}
```

### Alternative: Direct protoc Usage

For simpler setups without buf:

```bash
protoc --rust-aip_out=./src/gen \
       --prost_out=./src/gen \
       --tonic_out=./src/gen \
       your_proto_files.proto
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

The repository includes comprehensive examples demonstrating the integration patterns:

### Library Example (`examples/as-lib/`)

A complete library crate showing:
- Prost/tonic protobuf generation
- AIP resource name generation  
- prost-crate integration for features
- Proper module structure

### Test Application (`examples/test-app/`)

A test application that consumes the library and demonstrates:
- Using generated protobuf types
- Resource name creation and validation
- gRPC client setup
- Error handling

### Running the Examples

```bash
# Generate code and run tests
make examples

# Or manually:
cd examples/as-lib
buf generate
cd ../test-app
cargo run
```

The test application output shows:
```
Testing the refactored protoc-gen-rust-aip library...

--- Testing Basic Generated Types ---
Created Book: The Rust Programming Language by Steve Klabnik
Created Shelf: Programming Books
...

--- Testing AIP Resource Names ---
Book resource name: projects/my-project/books/rust-programming
Book resource type: library.googleapis.com/Book
Book name validation: PASSED
...
```

## Development

### Building from Source

```bash
# Install the plugin
make install

# Generate example code
make generate

# Run tests
make test test-rust

# Run examples
make examples

# Development workflow
make dev
```

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
