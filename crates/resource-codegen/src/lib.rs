mod emit_rust;
mod parse_descriptor;
mod render;

pub use emit_rust::EmitRust;
pub use parse_descriptor::{ParsedResource, ParsedResourceReference, ResourceParser};
pub use render::ResourceRenderer;

use anyhow::Result;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use std::collections::HashMap;

/// Configuration options for the resource code generator
#[derive(Debug, Clone)]
pub struct ResourceCodegenOptions {
    /// Whether to generate extension traits for prost types
    pub generate_extensions: bool,
    /// Output file suffix (e.g. "_resources.rs")
    pub file_suffix: String,
    /// Module prefix for generated code
    pub module_prefix: String,
}

impl Default for ResourceCodegenOptions {
    fn default() -> Self {
        Self {
            generate_extensions: true,
            file_suffix: "_resources.rs".to_string(),
            module_prefix: "".to_string(),
        }
    }
}

impl ResourceCodegenOptions {
    /// Parse options from protoc parameter string
    pub fn from_parameter(parameter: &str) -> Result<Self> {
        let mut options = Self::default();

        if parameter.is_empty() {
            return Ok(options);
        }

        for part in parameter.split(',') {
            let parts: Vec<&str> = part.splitn(2, '=').collect();
            match parts.as_slice() {
                ["generate_extensions", value] => {
                    options.generate_extensions = value.parse()?;
                }
                ["file_suffix", value] => {
                    options.file_suffix = value.to_string();
                }
                ["module_prefix", value] => {
                    options.module_prefix = value.to_string();
                }
                [key] if key.is_empty() => {} // Skip empty
                [key] => {
                    // Handle boolean flags without values
                    match *key {
                        "generate_extensions" => options.generate_extensions = true,
                        _ => anyhow::bail!("Unknown option: {}", key),
                    }
                }
                _ => anyhow::bail!("Invalid option format: {}", part),
            }
        }

        Ok(options)
    }
}

/// Main code generator for protobuf resource names
pub struct ResourceCodegen {
    options: ResourceCodegenOptions,
    parser: ResourceParser,
    renderer: ResourceRenderer,
}

impl ResourceCodegen {
    pub fn new(options: ResourceCodegenOptions) -> Self {
        Self {
            options,
            parser: ResourceParser::new(),
            renderer: ResourceRenderer::new(),
        }
    }

    pub fn generate(&self, request: CodeGeneratorRequest) -> Result<CodeGeneratorResponse> {
        // Parse all files to find resources and references
        let mut resources = HashMap::new();
        let mut references = HashMap::new();

        for file in &request.proto_file {
            let file_resources = self.parser.parse_file(file)?;
            for resource in file_resources.resources {
                resources.insert(resource.type_name.clone(), resource);
            }
            for reference in file_resources.references {
                references
                    .entry(reference.containing_message.clone())
                    .or_insert_with(Vec::new)
                    .push(reference);
            }
        }

        // Generate code for each file that contains resources or references
        let mut response = CodeGeneratorResponse::default();

        for file in &request.proto_file {
            if let Some(file_to_generate) = request
                .file_to_generate
                .iter()
                .find(|f| **f == file.name.as_ref().unwrap_or(&String::new()).as_str())
            {
                let file_resources: Vec<_> = resources
                    .values()
                    .filter(|r| r.source_file == *file.name.as_ref().unwrap_or(&String::new()))
                    .cloned()
                    .collect();

                let file_references: Vec<_> = references
                    .values()
                    .flatten()
                    .filter(|r| r.source_file == *file.name.as_ref().unwrap_or(&String::new()))
                    .cloned()
                    .collect();

                if !file_resources.is_empty() || !file_references.is_empty() {
                    let output_filename = self.get_output_filename(file_to_generate);
                    let content = self.renderer.render_file(
                        &file_resources,
                        &file_references,
                        &self.options,
                    )?;

                    let mut output_file =
                        prost_types::compiler::code_generator_response::File::default();
                    output_file.name = Some(output_filename);
                    output_file.content = Some(content);
                    response.file.push(output_file);
                }
            }
        }

        Ok(response)
    }

    fn get_output_filename(&self, proto_file: &str) -> String {
        let base = proto_file.strip_suffix(".proto").unwrap_or(proto_file);
        format!("{}{}", base, self.options.file_suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_parsing() {
        let options = ResourceCodegenOptions::from_parameter("").unwrap();
        assert_eq!(options.generate_extensions, true);
        assert_eq!(options.file_suffix, "_resources.rs");

        let options =
            ResourceCodegenOptions::from_parameter("generate_extensions=false,file_suffix=_rn.rs")
                .unwrap();
        assert_eq!(options.generate_extensions, false);
        assert_eq!(options.file_suffix, "_rn.rs");
    }
}
