#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A component of a resource pattern
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResourcePatternComponent {
    /// A literal string component (e.g., "projects/", "/topics/")
    Literal(String),
    /// A variable component (e.g., "project", "topic")
    Variable(String),
}

/// A parsed resource name pattern
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResourcePattern {
    /// The original pattern string (e.g., "projects/{project}/topics/{topic}")
    pub pattern: String,
    /// The parsed components
    pub components: Vec<ResourcePatternComponent>,
}

impl ResourcePattern {
    /// Create a new resource pattern
    pub fn new(pattern: String, components: Vec<ResourcePatternComponent>) -> Self {
        Self {
            pattern,
            components,
        }
    }

    /// Get all variable names in this pattern
    pub fn variables(&self) -> Vec<&str> {
        self.components
            .iter()
            .filter_map(|component| match component {
                ResourcePatternComponent::Variable(name) => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Check if this pattern contains a specific variable
    pub fn has_variable(&self, var_name: &str) -> bool {
        self.variables().contains(&var_name)
    }

    /// Get the number of variables in this pattern
    pub fn variable_count(&self) -> usize {
        self.variables().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_variables() {
        let pattern = ResourcePattern {
            pattern: "projects/{project}/topics/{topic}".to_string(),
            components: vec![
                ResourcePatternComponent::Literal("projects/".to_string()),
                ResourcePatternComponent::Variable("project".to_string()),
                ResourcePatternComponent::Literal("/topics/".to_string()),
                ResourcePatternComponent::Variable("topic".to_string()),
            ],
        };

        let variables = pattern.variables();
        assert_eq!(variables, vec!["project", "topic"]);
        assert_eq!(pattern.variable_count(), 2);
        assert!(pattern.has_variable("project"));
        assert!(pattern.has_variable("topic"));
        assert!(!pattern.has_variable("bucket"));
    }
}
