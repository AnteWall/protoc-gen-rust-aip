# Examples

This directory contains a comprehensive example demonstrating all the features of the `protoc-gen-rn` Rust protobuf resource name generator.

## Comprehensive Example

The `comprehensive/` directory contains a single unified example that showcases all resource name patterns and features:

- **Basic resource patterns** - Simple patterns with variable substitution (Project, Topic, Bucket)
- **Collection ID patterns** - Nested resources with collection identifiers (User, Document) 
- **Multi-pattern resources** - Resources supporting multiple valid naming patterns (Instance)
- **Complex hierarchical patterns** - Deep nesting with multiple components (Database)
- **Resource references** - Cross-resource relationships and typed field access
- **Extension traits** - Generated helper methods for typed resource name access

### Running the Example

```bash
cd examples/comprehensive
buf generate
cargo run --example comprehensive_usage
```

### What's Generated

The example generates:
- 8 different resource name types (`ProjectName`, `TopicName`, `UserName`, etc.)
- Pattern parsing and validation for each resource type
- Extension traits providing typed access to protobuf message fields
- Round-trip conversion between strings and typed resource names
- Component extraction methods for each resource pattern

### Output

The example produces output demonstrating:
1. Basic pattern usage and component extraction
2. Collection ID pattern handling  
3. Multi-pattern resource parsing (with current parser limitations noted)
4. Resource reference handling and cross-resource relationships

This comprehensive example replaces the previous separate examples that were scattered across multiple directories, providing a unified demonstration of all capabilities in one place.
