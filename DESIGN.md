# Architecture and Design

This document describes the architecture and design principles of protoc-gen-rust-aip.

## Overview

protoc-gen-rust-aip is a protoc plugin that generates zero-cost, type-safe Rust helpers for Google API resource names. It parses protobuf files annotated with `google.api.resource` and `google.api.resource_reference` annotations and generates idiomatic Rust code.

## Design Goals

1. **Zero-cost abstractions**: Generated types should compile to simple strings
2. **Type safety**: Prevent resource name misuse at compile time
3. **Ergonomic APIs**: Provide convenient methods for common operations
4. **prost compatibility**: Work seamlessly with prost-generated structs
5. **AIP compliance**: Follow Google API Improvement Proposals

## Architecture

### Core Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  protoc-gen-rn  │───▶│ resource-codegen │───▶│ resource-types  │
│    (binary)     │    │   (library)     │    │   (runtime)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

#### protoc-gen-rn (Binary)

The main protoc plugin executable that:
- Reads `CodeGeneratorRequest` from stdin
- Parses plugin parameters
- Delegates to `resource-codegen` for code generation
- Writes `CodeGeneratorResponse` to stdout

#### resource-codegen (Library)

The core code generation library with these modules:

- **`parse_descriptor`**: Extracts resource definitions from protobuf descriptors
- **`render`**: Generates Rust code tokens from parsed resources
- **`emit_rust`**: High-level code emission interface
- **`lib`**: Main orchestration and plugin options

#### resource-types (Runtime)

Runtime types and traits used by generated code:

- **Error types**: `ParseError` for validation failures
- **Pattern types**: `ResourcePattern`, `ResourcePatternComponent`
- **Traits**: `ResourceName`, `ResourceNameExt`, etc.

### Data Flow

```
Proto Files → protoc → CodeGeneratorRequest → Parser → Resources → Renderer → Rust Code
     ↓              ↑                            ↓         ↓         ↓
Annotations  Plugin Binary                 Validation  Templates  Output Files
```

1. **Input**: Protobuf files with Google API resource annotations
2. **Parsing**: Extract resource and reference definitions
3. **Validation**: Ensure patterns and types are valid
4. **Generation**: Create Rust types and implementations
5. **Output**: Rust source files with helper types

### Generated Code Structure

For each resource type, the generator creates:

```rust
// Core type (newtype around String)
pub struct ResourceName { inner: String }

// Constructor and accessors
impl ResourceName {
    pub fn new(components...) -> Self { ... }
    pub fn component(&self) -> &str { ... }
}

// Standard traits
impl Display for ResourceName { ... }
impl TryFrom<String> for ResourceName { ... }
impl FromStr for ResourceName { ... }

// Custom traits
impl ResourceName for ResourceName {
    fn pattern() -> &'static str { ... }
    fn as_str(&self) -> &str { ... }
}
```

For protobuf messages with resource references:

```rust
// Extension trait for type-safe field access
pub trait MessageExt {
    fn resource_field_typed(&self) -> Result<ResourceName, ParseError>;
    fn set_resource_field_typed(&mut self, value: ResourceName);
}

impl MessageExt for GeneratedMessage { ... }
```

## Pattern Parsing

Resource patterns like `projects/{project}/topics/{topic}` are parsed into:

```rust
ResourcePattern {
    pattern: "projects/{project}/topics/{topic}",
    components: vec![
        Literal("projects/"),
        Variable("project"),
        Literal("/topics/"),
        Variable("topic"),
    ],
}
```

This enables:
- Pattern validation during parsing
- Component extraction methods
- Format string generation for constructors

## Code Generation Strategy

### Template-based Generation

The renderer uses `quote!` macros to generate Rust code:

```rust
let type_ident = format_ident!("{}", type_name);
let tokens = quote! {
    pub struct #type_ident {
        inner: String,
    }
    
    impl #type_ident {
        // Generated methods...
    }
};
```

### Multi-pattern Support

Resources can have multiple patterns (e.g., project-level and organization-level):

```protobuf
message Database {
  option (google.api.resource) = {
    type: "spanner.googleapis.com/Database"
    pattern: "projects/{project}/instances/{instance}/databases/{database}"
    pattern: "organizations/{organization}/instances/{instance}/databases/{database}"
  };
}
```

This generates constructors and parsers for each pattern variant.

### Extension Traits

For messages with resource reference fields, extension traits provide type-safe access:

```rust
// Instead of: message.topic_name = "projects/p/topics/t".to_string()
// Use: message.set_topic_name_typed(TopicName::new("p", "t"))
```

## Error Handling

The `ParseError` type covers validation failures:

```rust
pub enum ParseError {
    InvalidPattern { value, expected_pattern },
    MissingComponent { component, value },
    InvalidComponent { component, component_value, value },
    EmptyValue,
    InvalidFormat { reason },
}
```

## Testing Strategy

### Unit Tests

- Pattern parsing: Verify regex matching and component extraction
- Type generation: Test type name derivation and formatting
- Option parsing: Validate plugin parameter handling

### Integration Tests

- End-to-end: Run plugin on test proto files
- Generated code: Compile and test generated Rust code
- Error cases: Verify proper error handling

### Example Tests

- Working examples: Demonstrate real-world usage
- Feature coverage: Each AIP feature has an example
- Documentation: Examples serve as user documentation

## Performance Considerations

### Zero-cost Design

Generated types are newtypes around `String`:

```rust
pub struct TopicName { inner: String }
```

At runtime, this compiles to just a `String` with no overhead.

### Compile-time Validation

Pattern validation happens at compile time for literal patterns:

```rust
const PATTERN: &str = "projects/{project}/topics/{topic}";
```

### Minimal Dependencies

Runtime dependencies are minimal:
- No heavy parsing libraries
- No regex at runtime (patterns are pre-parsed)
- Optional serde support behind feature flag

## Future Enhancements

### Advanced Pattern Features

- Pattern inheritance from parent resources
- Collection ID validation (e.g., RFC 1123 compliance)
- Wildcard patterns and matching

### Code Generation Improvements

- Custom derive macros for user types
- Async-friendly APIs
- Integration with other protobuf generators

### Tooling Integration

- IDE support and completion
- Linting rules for resource patterns
- Migration tools for pattern changes

## AIP Compliance

The generator follows these Google API Improvement Proposals:

- **AIP-122**: Resource name patterns and formats
- **AIP-123**: Resource type definitions and annotations
- **AIP-124**: Resource reference field annotations
- **AIP-148**: Standard field definitions

## Comparison with Other Implementations

### vs. Go generator

- **Rust**: Zero-cost newtypes, compile-time validation
- **Go**: Interface-based, runtime validation

### vs. Manual implementation

- **Generated**: Consistent, validated, feature-complete
- **Manual**: Error-prone, incomplete, inconsistent

### vs. String-based

- **Typed**: Compile-time safety, component access
- **String**: Runtime errors, manual parsing

This architecture provides a robust foundation for type-safe resource name handling in Rust while maintaining the performance characteristics expected in systems programming.
