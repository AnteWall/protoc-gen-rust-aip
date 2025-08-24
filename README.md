# protoc-gen-rust-aip

A Rust code generator that produces zero-cost helpers for protobuf Resource Names based on `google.api.resource` and `google.api.resource_reference` annotations. The generator runs as a **protoc/buf plugin** and outputs idiomatic Rust types + utilities that work seamlessly with `prost`-generated structs.

> **🚀 Like `protoc-gen-go-aip` for Rust**  
> This is the Rust equivalent of [protoc-gen-go-aip](https://github.com/einride/aip-go). Install with `cargo install protoc-gen-rust-aip` just like `go install go.einride.tech/aip/cmd/protoc-gen-go-aip`.

## Quick Start

### Installation

Choose your preferred method:

```bash
# Option 1: Install from source (requires Rust)
cargo install protoc-gen-rust-aip

# Option 2: Download binary (Linux/macOS)
curl -sSL https://raw.githubusercontent.com/protoc-gen-rust-aip/protoc-gen-rust-aip/main/install.sh | bash

# Option 3: Manual download
curl -L "https://github.com/protoc-gen-rust-aip/protoc-gen-rust-aip/releases/latest/download/protoc-gen-rust-aip-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)" -o protoc-gen-rust-aip
chmod +x protoc-gen-rust-aip
sudo mv protoc-gen-rust-aip /usr/local/bin/
```

### Usage with Buf

Add to your `buf.gen.yaml`:

```yaml
version: v2
plugins:
  - local: protoc-gen-rust-aip
    out: gen/rust
    opt:
      - paths=source_relative
```

### Usage with protoc

```bash
protoc --rust-aip_out=gen/rust --rust_out=gen/rust your_file.proto
```

## Features

- **Zero-cost abstractions**: Generated types are newtypes that compile to simple strings
- **Type safety**: Compile-time validation of resource name patterns
- **Seamless integration**: Works with existing `prost`-generated code
- **Pattern matching**: Support for multiple resource patterns per type
- **Validation**: Runtime parsing and validation of resource names
- **Buf integration**: Works as a protoc plugin with buf

## Example

### Proto Definition

```protobuf
syntax = "proto3";

import "google/api/resource.proto";

message Topic {
  option (google.api.resource) = {
    type: "pubsub.googleapis.com/Topic"
    pattern: "projects/{project}/topics/{topic}"
  };
  
  string name = 1 [(google.api.resource_reference) = {
    type: "pubsub.googleapis.com/Topic"
  }];
}
```

### Generated Rust Code Usage

```rust
use generated::pubsub::TopicName;

// Parse from string
let topic = TopicName::try_from("projects/my-project/topics/my-topic")?;

// Access components
println!("Project: {}", topic.project());
println!("Topic: {}", topic.topic());

// Convert back to string
let name_str: String = topic.into();
```

## Complete Example

### 1. Setup your project

```bash
# Initialize buf configuration
buf config init

# Create your proto files directory
mkdir -p proto
```

### 2. Configure buf.yaml

```yaml
version: v2
modules:
  - path: proto
deps:
  - buf.build/googleapis/googleapis
```

### 3. Configure buf.gen.yaml

```yaml
version: v2
plugins:
  - local: protoc-gen-rust-aip
    out: gen/rust
    opt:
      - paths=source_relative
  - remote: buf.build/community/neoeinstein-prost
    out: gen/rust
    opt: 
      - bytes=.
```

### 4. Add resource annotations to your proto files

```protobuf
syntax = "proto3";

import "google/api/resource.proto";

message Topic {
  option (google.api.resource) = {
    type: "pubsub.googleapis.com/Topic"
    pattern: "projects/{project}/topics/{topic}"
  };
  
  string name = 1 [(google.api.resource_reference) = {
    type: "pubsub.googleapis.com/Topic"
  }];
}
```

### 5. Generate code

```bash
buf generate
```

### 6. Use the generated types

```rust
use generated::pubsub::TopicName;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse from string
    let topic = TopicName::try_from("projects/my-project/topics/my-topic")?;

    // Access components
    println!("Project: {}", topic.project());
    println!("Topic: {}", topic.topic());

    // Convert back to string
    let name_str: String = topic.into();
    println!("Full name: {}", name_str);
    
    Ok(())
}
```

## How it works

The plugin generates zero-cost Rust types for protobuf resource names. For each resource, you get:

- **Type-safe wrappers**: `TopicName` instead of `String`
- **Pattern validation**: Ensures resource names match their patterns
- **Component access**: Extract parts like `project()` and `topic()` 
- **Standard traits**: `TryFrom<String>`, `Display`, `AsRef<str>`

## Examples

The `examples/` directory contains working examples:

```bash
# Try the basic example
cd examples/basic-resource
buf generate
cargo run --example basic_usage
```

Available examples:
- `basic-resource/`: Simple resource with single pattern
- `multi-pattern/`: Resource with multiple patterns  
- `resource-reference/`: Cross-resource references
- `collection-ids/`: Collection identifiers

## Development

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Test with example
cd examples/basic-resource
buf generate
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
