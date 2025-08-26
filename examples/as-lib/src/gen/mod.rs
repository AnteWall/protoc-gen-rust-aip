// @generated
pub mod example {
    pub mod bookstore {
        #[cfg(feature = "example-bookstore-v1")]
        // @@protoc_insertion_point(attribute:example.bookstore.v1)
        pub mod v1 {
            include!("example.bookstore.v1.rs");
            // @@protoc_insertion_point(example.bookstore.v1)
        }
    }
    pub mod library {
        #[cfg(feature = "example-library-v1")]
        // @@protoc_insertion_point(attribute:example.library.v1)
        pub mod v1 {
            include!("example.library.v1.rs");
            // @@protoc_insertion_point(example.library.v1)
        }
    }
    pub mod optional {
        #[cfg(feature = "example-optional-v1")]
        // @@protoc_insertion_point(attribute:example.optional.v1)
        pub mod v1 {
            include!("example.optional.v1.rs");
            // @@protoc_insertion_point(example.optional.v1)
        }
    }
}