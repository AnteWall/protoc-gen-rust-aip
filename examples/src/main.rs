use comprehensive_example::run_all_examples;

fn main() {
    match run_all_examples() {
        Ok(()) => {
            println!("🎉 All resource name examples completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ Error running examples: {e}");
            std::process::exit(1);
        }
    }
}
