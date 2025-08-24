use anyhow::Result;
use prost_types::{DescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use regex::Regex;
use resource_types::{ResourcePattern, ResourcePatternComponent};

/// A parsed resource definition from protobuf annotations
#[derive(Debug, Clone)]
pub struct ParsedResource {
    /// The resource type (e.g., "pubsub.googleapis.com/Topic")
    pub type_name: String,
    /// The patterns for this resource (e.g., "projects/{project}/topics/{topic}")
    pub patterns: Vec<ResourcePattern>,
    /// The singular form (e.g., "topic")
    pub singular: Option<String>,
    /// The plural form (e.g., "topics")
    pub plural: Option<String>,
    /// Whether this resource has history
    pub history: Vec<String>,
    /// The source proto file
    pub source_file: String,
    /// The message name that declares this resource
    pub message_name: String,
}

/// A parsed resource reference from a field annotation
#[derive(Debug, Clone)]
pub struct ParsedResourceReference {
    /// The resource type this field references
    pub resource_type: Option<String>,
    /// The child resource type if this is a parent reference
    pub child_type: Option<String>,
    /// The field name
    pub field_name: String,
    /// The message containing this field
    pub containing_message: String,
    /// The source proto file
    pub source_file: String,
}

/// Collection of parsed resources and references from a file
#[derive(Debug, Default)]
pub struct ParsedFileResources {
    pub resources: Vec<ParsedResource>,
    pub references: Vec<ParsedResourceReference>,
}

/// Parser for extracting resource information from protobuf descriptors
pub struct ResourceParser {
    pattern_regex: Regex,
}

impl ResourceParser {
    pub fn new() -> Self {
        Self {
            // Regex to match resource pattern variables like {project}, {topic}
            pattern_regex: Regex::new(r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}").unwrap(),
        }
    }

    /// Parse a file descriptor for resource definitions and references
    pub fn parse_file(&self, file: &FileDescriptorProto) -> Result<ParsedFileResources> {
        let mut result = ParsedFileResources::default();
        let file_name = file.name.as_ref().unwrap_or(&String::new()).clone();

        // Parse messages in the file
        for message in &file.message_type {
            self.parse_message(message, &file_name, "", &mut result)?;
        }

        Ok(result)
    }

    fn parse_message(
        &self,
        message: &DescriptorProto,
        file_name: &str,
        parent_prefix: &str,
        result: &mut ParsedFileResources,
    ) -> Result<()> {
        let message_name = format!(
            "{}{}",
            parent_prefix,
            message.name.as_ref().unwrap_or(&String::new())
        );

        // Check for google.api.resource annotation
        if let Some(resource) = self.parse_resource_annotation(message, file_name, &message_name)? {
            result.resources.push(resource);
        }

        // Parse fields for resource references
        for field in &message.field {
            if let Some(reference) =
                self.parse_resource_reference_annotation(field, file_name, &message_name)?
            {
                result.references.push(reference);
            }
        }

        // Parse nested messages
        for nested in &message.nested_type {
            self.parse_message(nested, file_name, &format!("{message_name}."), result)?;
        }

        Ok(())
    }

    fn parse_resource_annotation(
        &self,
        message: &DescriptorProto,
        file_name: &str,
        message_name: &str,
    ) -> Result<Option<ParsedResource>> {
        let options = message.options.as_ref();
        if options.is_none() {
            return Ok(None);
        }

        let options = options.unwrap();

        // Look for google.api.resource extension (field number 1053)
        // This is the proper way to detect resource annotations
        for uninterpreted_option in &options.uninterpreted_option {
            if let Some(resource) = self.parse_google_api_resource_option(
                uninterpreted_option,
                file_name,
                message_name,
            )? {
                return Ok(Some(resource));
            }
        }

        Ok(None)
    }

    fn parse_google_api_resource_option(
        &self,
        option: &prost_types::UninterpretedOption,
        file_name: &str,
        message_name: &str,
    ) -> Result<Option<ParsedResource>> {
        // Check if this is a google.api.resource option by examining the name parts
        // For now, we'll use a simplified check - in a real implementation this would
        // properly decode the option field number (1053 for google.api.resource)
        let is_resource_option = !option.name.is_empty();

        if !is_resource_option {
            return Ok(None);
        }

        // Parse the actual resource descriptor from the option value
        // The option value contains a serialized google.api.ResourceDescriptor
        if let Some(aggregate_value) = &option.aggregate_value {
            return self.parse_resource_descriptor_from_aggregate(
                aggregate_value,
                file_name,
                message_name,
            );
        }

        // If there's no aggregate value, we can't parse the resource
        Ok(None)
    }

    fn parse_resource_descriptor_from_aggregate(
        &self,
        aggregate_value: &str,
        file_name: &str,
        message_name: &str,
    ) -> Result<Option<ParsedResource>> {
        // Parse the protobuf text format for ResourceDescriptor
        // Format: type: "domain/Type" pattern: "collection/{id}" singular: "name" plural: "names"

        let mut type_name: Option<String> = None;
        let mut patterns: Vec<String> = Vec::new();
        let mut singular: Option<String> = None;
        let mut plural: Option<String> = None;
        let mut history: Vec<String> = Vec::new();

        // Simple text format parser for the aggregate value
        for line in aggregate_value.lines() {
            let line = line.trim();

            if let Some(value) = self.extract_quoted_value(line, "type:") {
                type_name = Some(value);
            } else if let Some(value) = self.extract_quoted_value(line, "pattern:") {
                patterns.push(value);
            } else if let Some(value) = self.extract_quoted_value(line, "singular:") {
                singular = Some(value);
            } else if let Some(value) = self.extract_quoted_value(line, "plural:") {
                plural = Some(value);
            } else if let Some(value) = self.extract_quoted_value(line, "history:") {
                history.push(value);
            }
        }

        // Validate that we have the required fields
        let type_name = type_name
            .ok_or_else(|| anyhow::anyhow!("Resource annotation missing required 'type' field"))?;

        if patterns.is_empty() {
            return Err(anyhow::anyhow!(
                "Resource annotation missing required 'pattern' field"
            ));
        }

        // Parse patterns into ResourcePattern structs
        let parsed_patterns: Result<Vec<_>> =
            patterns.iter().map(|p| self.parse_pattern(p)).collect();

        Ok(Some(ParsedResource {
            type_name,
            patterns: parsed_patterns?,
            singular,
            plural,
            history,
            source_file: file_name.to_string(),
            message_name: message_name.to_string(),
        }))
    }

    fn extract_quoted_value(&self, line: &str, prefix: &str) -> Option<String> {
        if line.starts_with(prefix) {
            let value_part = line.strip_prefix(prefix)?.trim();
            // Remove quotes if present (both single and double quotes)
            if (value_part.starts_with('"') && value_part.ends_with('"'))
                || (value_part.starts_with('\'') && value_part.ends_with('\''))
            {
                return Some(value_part[1..value_part.len() - 1].to_string());
            } else {
                // Handle unquoted values
                return Some(value_part.to_string());
            }
        }
        None
    }

    fn parse_resource_reference_annotation(
        &self,
        field: &FieldDescriptorProto,
        file_name: &str,
        message_name: &str,
    ) -> Result<Option<ParsedResourceReference>> {
        let options = field.options.as_ref();
        if options.is_none() {
            return Ok(None);
        }

        let empty_string = String::new();
        let field_name = field.name.as_ref().unwrap_or(&empty_string);

        // Only process string fields that have options
        if field.r#type() != prost_types::field_descriptor_proto::Type::String {
            return Ok(None);
        }

        let options = options.unwrap();

        // Look for google.api.resource_reference extension
        for uninterpreted_option in &options.uninterpreted_option {
            if let Some(reference) = self.parse_google_api_resource_reference_option(
                uninterpreted_option,
                field_name,
                message_name,
                file_name,
            )? {
                return Ok(Some(reference));
            }
        }

        Ok(None)
    }

    fn parse_google_api_resource_reference_option(
        &self,
        option: &prost_types::UninterpretedOption,
        field_name: &str,
        message_name: &str,
        file_name: &str,
    ) -> Result<Option<ParsedResourceReference>> {
        // Check if this is a google.api.resource_reference option
        // For now, we'll use a simplified check - in a real implementation this would
        // properly decode the option field number
        let is_resource_reference_option = !option.name.is_empty();

        if !is_resource_reference_option {
            return Ok(None);
        }

        // Parse the resource reference from the aggregate value
        if let Some(aggregate_value) = &option.aggregate_value {
            return self.parse_resource_reference_from_aggregate(
                aggregate_value,
                field_name,
                message_name,
                file_name,
            );
        }

        Ok(None)
    }

    fn parse_resource_reference_from_aggregate(
        &self,
        aggregate_value: &str,
        field_name: &str,
        message_name: &str,
        file_name: &str,
    ) -> Result<Option<ParsedResourceReference>> {
        // Parse the resource reference from protobuf text format
        // Format: type: "domain/Type" or child_type: "domain/ChildType"

        let mut resource_type: Option<String> = None;
        let mut child_type: Option<String> = None;

        for line in aggregate_value.lines() {
            let line = line.trim();

            if let Some(value) = self.extract_quoted_value(line, "type:") {
                resource_type = Some(value);
            } else if let Some(value) = self.extract_quoted_value(line, "child_type:") {
                child_type = Some(value);
            }
        }

        // At least one of resource_type or child_type must be specified
        if resource_type.is_none() && child_type.is_none() {
            return Ok(None);
        }

        Ok(Some(ParsedResourceReference {
            resource_type,
            child_type,
            field_name: field_name.to_string(),
            containing_message: message_name.to_string(),
            source_file: file_name.to_string(),
        }))
    }

    fn parse_pattern(&self, pattern: &str) -> Result<ResourcePattern> {
        let mut components = Vec::new();
        let mut last_end = 0;

        for cap in self.pattern_regex.captures_iter(pattern) {
            let full_match = cap.get(0).unwrap();
            let var_name = cap.get(1).unwrap().as_str();

            // Add literal text before this variable
            if full_match.start() > last_end {
                let literal = &pattern[last_end..full_match.start()];
                if !literal.is_empty() {
                    components.push(ResourcePatternComponent::Literal(literal.to_string()));
                }
            }

            // Add the variable
            components.push(ResourcePatternComponent::Variable(var_name.to_string()));
            last_end = full_match.end();
        }

        // Add any remaining literal text
        if last_end < pattern.len() {
            let literal = &pattern[last_end..];
            if !literal.is_empty() {
                components.push(ResourcePatternComponent::Literal(literal.to_string()));
            }
        }

        Ok(ResourcePattern {
            pattern: pattern.to_string(),
            components,
        })
    }
}

impl Default for ResourceParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pattern() {
        let parser = ResourceParser::new();
        let pattern = parser
            .parse_pattern("projects/{project}/topics/{topic}")
            .unwrap();

        assert_eq!(pattern.pattern, "projects/{project}/topics/{topic}");
        assert_eq!(pattern.components.len(), 4);

        match &pattern.components[0] {
            ResourcePatternComponent::Literal(s) => assert_eq!(s, "projects/"),
            _ => panic!("Expected literal"),
        }

        match &pattern.components[1] {
            ResourcePatternComponent::Variable(s) => assert_eq!(s, "project"),
            _ => panic!("Expected variable"),
        }
    }

    #[test]
    fn test_parse_simple_pattern() {
        let parser = ResourceParser::new();
        let pattern = parser.parse_pattern("users/{user}").unwrap();

        assert_eq!(pattern.components.len(), 2);
        match &pattern.components[0] {
            ResourcePatternComponent::Literal(s) => assert_eq!(s, "users/"),
            _ => panic!("Expected literal"),
        }
        match &pattern.components[1] {
            ResourcePatternComponent::Variable(s) => assert_eq!(s, "user"),
            _ => panic!("Expected variable"),
        }
    }
}
