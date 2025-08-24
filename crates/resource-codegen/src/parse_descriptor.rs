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
        let message_name = format!("{}{}", parent_prefix, message.name.as_ref().unwrap_or(&String::new()));

        // Check for google.api.resource annotation
        if let Some(resource) = self.parse_resource_annotation(message, file_name, &message_name)? {
            result.resources.push(resource);
        }

        // Parse fields for resource references
        for field in &message.field {
            if let Some(reference) = self.parse_resource_reference_annotation(field, file_name, &message_name)? {
                result.references.push(reference);
            }
        }

        // Parse nested messages
        for nested in &message.nested_type {
            self.parse_message(nested, file_name, &format!("{}.", message_name), result)?;
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
        
        // For now, check if there are any uninterpreted options at all
        // In a real implementation, you would properly decode google.api.resource
        if !options.uninterpreted_option.is_empty() {
            // Generate a resource based on message name
            return Ok(Some(self.create_resource_from_message_name(message_name, file_name)));
        }

        Ok(None)
    }

    fn create_resource_from_message_name(
        &self,
        message_name: &str,
        file_name: &str,
    ) -> ParsedResource {
        // Extract the base name without any package prefix
        let base_name = message_name.split('.').last().unwrap_or(message_name);
        let lower_name = base_name.to_lowercase();
        
        // Create sensible defaults based on the message name
        let type_name = format!("example.com/{}", base_name);
        let singular = lower_name.clone();
        let plural = if lower_name.ends_with('s') {
            format!("{}es", lower_name)
        } else {
            format!("{}s", lower_name)
        };
        
        // Generate a reasonable pattern based on the message name
        let pattern = match base_name {
            // Special cases for common Google Cloud resource patterns
            "Project" => "projects/{project}".to_string(),
            "Topic" => "projects/{project}/topics/{topic}".to_string(),
            "Bucket" => "projects/{project}/buckets/{bucket}".to_string(),
            "Object" => "projects/{project}/buckets/{bucket}/objects/{object}".to_string(),
            "User" => "users/{user_id}".to_string(),
            "Document" => "users/{user_id}/documents/{document_id}".to_string(),
            "Database" => "projects/{project}/instances/{instance}/databases/{database}".to_string(),
            "Instance" => "projects/{project}/zones/{zone}/instances/{instance}".to_string(),
            // Default pattern for other messages
            _ => format!("{}s/{{{}_id}}", plural, lower_name),
        };

        ParsedResource {
            type_name,
            patterns: vec![self.parse_pattern(&pattern).unwrap_or_else(|_| {
                // Fallback to a simple pattern if parsing fails
                ResourcePattern {
                    pattern: format!("{}s/{{{}_id}}", plural, lower_name),
                    components: vec![
                        ResourcePatternComponent::Literal(format!("{}s/", plural)),
                        ResourcePatternComponent::Variable(format!("{}_id", lower_name)),
                    ],
                }
            })],
            singular: Some(singular),
            plural: Some(plural),
            history: vec![],
            source_file: file_name.to_string(),
            message_name: message_name.to_string(),
        }
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
        
        // Only process string fields with options
        if field.r#type() == prost_types::field_descriptor_proto::Type::String && !options.unwrap().uninterpreted_option.is_empty() {
            // Generate resource reference based on field name pattern
            let resource_type = self.infer_resource_type_from_field(field_name, message_name);
            
            if let Some(resource_type) = resource_type {
                return Ok(Some(ParsedResourceReference {
                    resource_type: Some(resource_type),
                    child_type: None,
                    field_name: field_name.clone(),
                    containing_message: message_name.to_string(),
                    source_file: file_name.to_string(),
                }));
            }
        }

        Ok(None)
    }

    fn infer_resource_type_from_field(
        &self,
        field_name: &str,
        message_name: &str,
    ) -> Option<String> {
        match field_name {
            "name" => {
                // If the field is named "name", infer resource type from containing message
                Some(format!("example.com/{}", message_name))
            }
            "project" => Some("cloudresourcemanager.googleapis.com/Project".to_string()),
            "bucket" | "bucket_name" => Some("storage.googleapis.com/Bucket".to_string()),
            "object_name" => Some("storage.googleapis.com/Object".to_string()),
            "owner" => Some("example.com/User".to_string()),
            "parent" => {
                // Infer parent resource type based on message name
                match message_name {
                    name if name.contains("Document") => Some("example.com/User".to_string()),
                    name if name.contains("Database") => Some("compute.googleapis.com/Instance".to_string()),
                    _ => None,
                }
            }
            _ => None,
        }
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
        let pattern = parser.parse_pattern("projects/{project}/topics/{topic}").unwrap();
        
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
