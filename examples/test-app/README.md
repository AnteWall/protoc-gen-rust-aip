# Test App for protoc-gen-rust-aip

This test application demonstrates the usage of the refactored `protoc-gen-rust-aip` library that works with tonic/prost generated protobuf code.

## What it tests

The application tests the following functionality:

### 1. Basic Generated Types
- `Book` - A simple resource with name, title, and author
- `Shelf` - A multi-pattern resource that can exist under projects or users
- `Author` - A resource with display name and biography
- `Review` - A nested resource that belongs to a book
- `Publisher` - A complex multi-pattern resource

### 2. AIP Resource Names
- `BookResourceName` - Single pattern resource name validation and formatting
- `ProjectsShelfResourceName` / `UsersShelfResourceName` - Multi-pattern resource names
- `ShelfResourceName` enum - Wrapper for different shelf resource name patterns
- Resource name parsing and validation

### 3. gRPC Client Types
- `ListBooksRequest` - Request for listing books with pagination
- `GetBookRequest` - Request for retrieving a specific book
- `GetResourceRequest` - Request for retrieving any resource

## Running the tests

```bash
cargo run
```

This will execute all the tests and verify that:
- All generated types can be created and used
- Resource name validation works correctly
- Resource name parsing and formatting works
- gRPC client types are properly generated

## Regenerating protobuf code

To regenerate the protobuf code in the `as-lib` dependency:

```bash
cd ../as-lib
buf generate
```

Then rebuild and test:

```bash
cd ../test-app
cargo run
```
