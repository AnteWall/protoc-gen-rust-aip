mod error;
mod pattern;
mod traits;

pub use error::ParseError;
pub use pattern::{ResourcePattern, ResourcePatternComponent};
pub use traits::ResourceName;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Marker trait for all generated resource name types
pub trait ResourceNameType: ResourceName + Clone + PartialEq + Eq + std::hash::Hash {}

// Blanket implementation for any type that implements the required traits
impl<T> ResourceNameType for T where T: ResourceName + Clone + PartialEq + Eq + std::hash::Hash {}
