use thiserror::Error;

/// Errors that can occur when parsing resource names
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error("Invalid pattern: got '{value}', expected pattern '{expected_pattern}'")]
    InvalidPattern {
        value: String,
        expected_pattern: String,
    },

    #[error("Missing required component '{component}' in resource name '{value}'")]
    MissingComponent { component: String, value: String },

    #[error(
        "Invalid component '{component}' value '{component_value}' in resource name '{value}'"
    )]
    InvalidComponent {
        component: String,
        component_value: String,
        value: String,
    },

    #[error("Resource name is empty")]
    EmptyValue,

    #[error("Invalid resource name format: {reason}")]
    InvalidFormat { reason: String },
}
