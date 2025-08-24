//! Traits for working with resource names

#![allow(dead_code)] // These are public API traits that may be unused in this crate

/// Trait implemented by all generated resource name types
pub trait ResourceName {
    /// Get the pattern string for this resource type
    fn pattern() -> &'static str
    where
        Self: Sized;

    /// Get the string representation of this resource name
    fn as_str(&self) -> &str;
}

/// Trait for types that can be validated as resource names
pub trait Validate {
    type Error;

    /// Validate that this resource name is well-formed
    fn validate(&self) -> Result<(), Self::Error>;
}

/// Trait for types that can be formatted as resource names
pub trait Format: ResourceName {
    /// Format this resource name for display
    fn format(&self) -> String {
        self.as_str().to_string()
    }
}

/// Extension trait providing utility methods for resource names
pub trait ResourceNameExt: ResourceName {
    /// Check if this resource name is empty
    fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Get the length of this resource name
    fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Check if this resource name starts with a prefix
    fn starts_with(&self, prefix: &str) -> bool {
        self.as_str().starts_with(prefix)
    }

    /// Check if this resource name ends with a suffix
    fn ends_with(&self, suffix: &str) -> bool {
        self.as_str().ends_with(suffix)
    }
}

// Blanket implementation for all ResourceName types
impl<T: ResourceName> ResourceNameExt for T {}

// Blanket implementation of Format for ResourceName types
impl<T: ResourceName> Format for T {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResourceName {
        inner: String,
    }

    impl ResourceName for TestResourceName {
        fn pattern() -> &'static str {
            "test/{id}"
        }

        fn as_str(&self) -> &str {
            &self.inner
        }
    }

    #[test]
    fn test_resource_name_ext() {
        let name = TestResourceName {
            inner: "test/123".to_string(),
        };

        assert!(!name.is_empty());
        assert_eq!(name.len(), 8);
        assert!(name.starts_with("test/"));
        assert!(name.ends_with("123"));
        assert_eq!(name.format(), "test/123");
    }
}
