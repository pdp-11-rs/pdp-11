use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.s> [output.bin]", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() > 2 { &args[2] } else { "a.out" };

    let _source = match fs::read_to_string(input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {input_file}: {e}");
            process::exit(1);
        }
    };

    println!("Assembling {input_file}...");
    println!("Output will be written to {output_file}");

    // TODO: Implement full assembly pipeline
    println!("\nNote: Assembler is under development");
    println!("Current features:");
    println!("  - Lexer: ✓");
    println!("  - Parser: Partial");
    println!("  - Encoder: Partial");
    println!("  - Symbol table: ✓");
    println!("  - Expression evaluator: ✓");
}
