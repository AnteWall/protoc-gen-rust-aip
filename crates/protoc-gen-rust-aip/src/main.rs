use anyhow::Result;
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use resource_codegen::{ResourceCodegen, ResourceCodegenOptions};
use std::env;
use std::io::{self, Read, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    // Handle version flag
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-V") {
        println!("protoc-gen-rust-aip {}", VERSION);
        return Ok(());
    }

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("protoc-gen-rust-aip {}", VERSION);
        println!("Generate Rust AIP helpers from protobuf resource annotations");
        println!();
        println!("This is a protoc plugin. Use it with protoc or buf:");
        println!("  protoc --rust-aip_out=. file.proto");
        println!("  buf generate");
        println!();
        println!("Options (via --rust-aip_opt):");
        println!("  paths=source_relative  Generate files relative to source");
        return Ok(());
    }

    // Read CodeGeneratorRequest from stdin
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let request = CodeGeneratorRequest::decode(&input[..])?;

    // Parse options from the parameter
    let options = ResourceCodegenOptions::from_parameter(
        request.parameter.as_ref().unwrap_or(&String::new()),
    )?;

    // Generate the code
    let codegen = ResourceCodegen::new(options);
    let response = codegen.generate(request)?;

    // Write CodeGeneratorResponse to stdout
    let output = response.encode_to_vec();
    io::stdout().write_all(&output)?;

    Ok(())
}
