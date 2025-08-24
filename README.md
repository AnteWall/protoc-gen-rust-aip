# protoc-gen-rust-aip

A Rust code generator that produces zero-cost helpers for protobuf Resource Names based on `google.api.resource` and `google.api.resource_reference` annotations. The generator runs as a **protoc/buf plugin** and outputs idiomatic Rust types + utilities that work seamlessly with `prost`-generated structs.

## Features

- **Zero-cost abstractions**: Generated types are newtypes that compile to simple strings
- **Type safety**: Compile-time validation of resource name patterns
- **Seamless integration**: Works with existing `prost`-generated code
- **Pattern matching**: Support for multiple resource patterns per type
- **Validation**: Runtime parsing and validation of resource names
- **Buf integration**: Works as a protoc plugin with buf

## Quick Start

### Using with Buf (Recommended)

1. Add the plugin to your `buf.gen.yaml`:

```yaml
version: v1
plugins:
  - plugin: buf.build/community/neoeinstein-prost:v0.4.0
    out: gen
    opt: 
      - bytes=.
      - file_descriptor_set=false
  - plugin: rn  # This plugin
    path: path/to/protoc-gen-rn  # For local development
    out: gen
    strategy: all
    opt:
      - generate_extensions=true
      - file_suffix=_resources.rs
```

2. Configure your `buf.yaml`:

```yaml
version: v2
modules:
  - path: proto
deps:
  - buf.build/googleapis/googleapis
```

3. Add resource annotations to your proto files:

3. Add resource annotations to your proto files:

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

4. Generate code:

```bash
buf generate
```

5. Use the generated types:

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

## Architecture

### Crates

- **`protoc-gen-rn`**: The main protoc plugin binary
- **`resource-codegen`**: Library with descriptor parsing & code emission
- **`resource-types`**: Runtime helpers and error types

### Generated Code Structure

For each resource type, the generator creates:

- A newtype wrapper (e.g., `TopicName`)
- Pattern-specific constructors and accessors  
- `TryFrom<String>`, `Display`, `AsRef<str>` implementations
- Validation and formatting utilities

## Examples

See the `examples/` directory for complete working examples of different AIP features:

- **`basic-resource`**: Simple resource with single pattern
- **`collection-ids`**: Resources with collection ID patterns  
- **`multi-pattern`**: Resource supporting multiple patterns
- **`resource-reference`**: Cross-resource references

To run the examples:

```bash
# Build the plugin
./scripts/build-and-test.sh

# Or manually:
cargo build
cd examples/basic-resource
buf dep update
buf generate
cargo run --example basic_usage
```

- `basic-resource/`: Simple resource with single pattern
- `multi-pattern/`: Resource with multiple patterns (parent/child relationships)
- `resource-reference/`: Using resource references in fields
- `collection-ids/`: Collection identifiers and pluralization

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
