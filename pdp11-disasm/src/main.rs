use clap::Parser;
use pdp11_common::Word;
use pdp11_core::insns::Instruction;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pdp11-disasm")]
#[command(about = "PDP-11 Disassembler - Convert machine code to MACRO-11 assembly", long_about = None)]
struct Args {
    /// Input file (binary, a.out, or raw machine code)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Start address for disassembly (octal)
    #[arg(short = 's', long, value_name = "ADDR")]
    start: Option<String>,

    /// Number of words to disassemble
    #[arg(short = 'n', long, value_name = "COUNT")]
    count: Option<usize>,

    /// Show addresses in output
    #[arg(short = 'a', long)]
    show_addresses: bool,

    /// Show opcodes in output
    #[arg(short = 'o', long)]
    show_opcodes: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read input (file or stdin)
    let data = if let Some(path) = &args.input {
        fs::read(path)?
    } else {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    };

    // Parse start address (octal by default)
    let start_addr = if let Some(start_str) = &args.start {
        let start_str = start_str.trim_start_matches("0o");
        usize::from_str_radix(start_str, 8).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid start address: {}", e),
            )
        })?
    } else {
        0
    };

    // Calculate how many words to disassemble
    let word_count = if let Some(count) = args.count {
        count
    } else {
        // Disassemble all data from start address
        (data.len().saturating_sub(start_addr)) / 2
    };

    // Ensure we don't read past the end
    let end_offset = start_addr + (word_count * 2);
    if end_offset > data.len() {
        eprintln!("Warning: Requested range exceeds data size, truncating");
    }

    // Disassemble
    let mut offset = start_addr;
    let mut pc = start_addr; // Track program counter for display
    let mut word_num = 0;

    while offset + 1 < data.len() && offset < end_offset && word_num < word_count {
        // Read word (little-endian)
        let low = data[offset];
        let high = data[offset + 1];
        let word_u16 = u16::from_le_bytes([low, high]);
        let word = Word::from(word_u16);

        // Decode instruction
        let instruction = Instruction::from(word);

        // Format output
        let mut output = String::new();

        if args.show_addresses {
            output.push_str(&format!("{:06o}:  ", pc));
        }

        if args.show_opcodes {
            output.push_str(&format!("{:06o}  ", word_u16));
        }

        output.push_str(&format!("{}", instruction));

        println!("{}", output);

        // Move to next word
        offset += 2;
        pc += 2;
        word_num += 1;
    }

    Ok(())
}
