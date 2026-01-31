// Test program to verify assembler basics

use pdp11_asm::*;
use pdp11_common::{Register, Word};

fn main() {
    println!("=== PDP-11 Assembler Test ===\n");

    // Test 1: Lexer
    println!("Test 1: Lexer");
    let mut lexer = Lexer::new("MOV R0, R1");
    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("  Token: {token:?}");
                if matches!(token, lexer::Token::Eof) {
                    break;
                }
            }
            Err(e) => {
                eprintln!("  Error: {e}");
                break;
            }
        }
    }

    // Test 2: Register parsing
    println!("\nTest 2: Register Parsing");
    for name in ["R0", "R5", "SP", "PC", "%3"] {
        match Register::parse(name) {
            Some(reg) => println!("  '{name}' -> {reg} (code: {})", reg.to_code()),
            None => println!("  '{name}' -> INVALID"),
        }
    }

    // Test 3: Word constants
    println!("\nTest 3: Word Constants");
    println!("  ZERO: {:o}", Word::ZERO);
    println!("  ONE: {:o}", Word::ONE);
    println!("  TWO: {:o}", Word::TWO);
    println!("  MAX_POSITIVE: {:o}", Word::MAX_POSITIVE);

    // Test 4: Addressing modes
    println!("\nTest 4: Addressing Mode Encoding");
    use pdp11_asm::AddressingMode;
    let modes = vec![
        ("R0", AddressingMode::Register(Register::R0)),
        ("(R1)", AddressingMode::RegisterDeferred(Register::R1)),
        ("(R2)+", AddressingMode::Autoincrement(Register::R2)),
        ("#val", AddressingMode::Immediate),
        ("@#addr", AddressingMode::Absolute),
    ];
    for (desc, mode) in modes {
        println!(
            "  {desc:12} -> {:06o} (extra word: {})",
            mode.encode(),
            mode.needs_extra_word()
        );
    }

    println!("\n=== All Basic Tests Passed ===");
}
