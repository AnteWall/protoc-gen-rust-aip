# Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── CONTRIBUTING.md
├── crates
│   ├── protoc-gen-rn
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── resource-codegen
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── emit_rust.rs
│   │       ├── lib.rs
│   │       ├── parse_descriptor.rs
│   │       └── render.rs
│   └── resource-types
│       ├── Cargo.toml
│       └── src
│           ├── error.rs
│           ├── lib.rs
│           ├── pattern.rs
│           └── traits.rs
├── DESIGN.md
├── examples
│   ├── basic-resource
│   │   ├── buf.gen.yaml
│   │   ├── buf.yaml
│   │   ├── Cargo.toml
│   │   ├── examples
│   │   │   └── basic_usage.rs
│   │   └── proto
│   │       └── topic.proto
│   ├── collection-ids
│   │   ├── buf.gen.yaml
│   │   ├── buf.yaml
│   │   └── proto
│   │       └── collections.proto
│   ├── multi-pattern
│   │   ├── buf.gen.yaml
│   │   ├── buf.yaml
│   │   └── proto
│   │       └── database.proto
│   ├── README.md
│   └── resource-reference
│       ├── buf.gen.yaml
│       ├── buf.yaml
│       └── proto
│           └── storage.proto
├── IMPLEMENTATION.md
├── LICENSE
├── PROJECT_STRUCTURE.md
├── README.md
└── scripts
    └── build-and-test.sh

18 directories, 36 files
```
