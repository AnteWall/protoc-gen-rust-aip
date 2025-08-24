use crate::{ParsedResource, ParsedResourceReference, ResourceCodegenOptions};
use anyhow::Result;

/// High-level interface for emitting Rust code
pub struct EmitRust;

impl EmitRust {
    pub fn new() -> Self {
        Self
    }

    /// Emit a complete Rust module for the given resources and references
    pub fn emit_module(
        &self,
        resources: &[ParsedResource],
        references: &[ParsedResourceReference],
        options: &ResourceCodegenOptions,
    ) -> Result<String> {
        // This would be implemented to coordinate with the renderer
        // For now, delegate to the renderer directly
        crate::ResourceRenderer::new().render_file(resources, references, options)
    }

    /// Emit individual resource type
    pub fn emit_resource(&self, resource: &ParsedResource) -> Result<String> {
        let renderer = crate::ResourceRenderer::new();
        let tokens = renderer.render_resource(resource)?;
        Ok(tokens.to_string())
    }
}

impl Default for EmitRust {
    fn default() -> Self {
        Self::new()
    }
}
