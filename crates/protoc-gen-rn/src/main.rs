use anyhow::Result;
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use resource_codegen::{ResourceCodegen, ResourceCodegenOptions};
use std::io::{self, Read, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Read CodeGeneratorRequest from stdin
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    
    let request = CodeGeneratorRequest::decode(&input[..])?;
    
    // Parse options from the parameter
    let options = ResourceCodegenOptions::from_parameter(
        &request.parameter.as_ref().unwrap_or(&String::new())
    )?;
    
    // Generate the code
    let codegen = ResourceCodegen::new(options);
    let response = codegen.generate(request)?;
    
    // Write CodeGeneratorResponse to stdout
    let output = response.encode_to_vec();
    io::stdout().write_all(&output)?;
    
    Ok(())
}
