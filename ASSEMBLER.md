# PDP-11 Assembler Implementation Summary

## Overview

Created a MACRO-11 compatible assembler as a separate crate with shared code reuse from the emulator.

## Architecture

### Crate Structure

```
pdp-11/
├── pdp11-common/         [NEW] Shared types library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── types.rs      Word and Byte types
│       └── register.rs   Register enum
│
├── pdp11-asm/            [NEW] Assembler library
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs        Main types (Instruction, Directive, etc.)
│       ├── addressing.rs AddressingMode enum
│       ├── encoder.rs    Instruction encoding
│       ├── error.rs      Error types
│       ├── expr.rs       Expression evaluator
│       ├── lexer.rs      Tokenization
│       ├── parser.rs     Parsing (partial)
│       └── symbol.rs     Symbol table
│
├── pdp11-asm-cli/        [NEW] Assembler CLI binary
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
│
├── pdp11-core/           [EXISTING] Emulator library
│   └── (unchanged - will be updated to use pdp11-common)
│
└── pdp11/                [EXISTING] Emulator binary
    └── (unchanged)
```

## Code Reuse Strategy

### pdp11-common (Shared Library)

**Purpose**: Types and utilities shared between emulator and assembler

**Exports**:
- `Word` - 16-bit word with PDP-11 semantics
  - Constants: ZERO, ONE, TWO, MAX_POSITIVE, MIN_NEGATIVE, MAX_UNSIGNED
  - Operations: Add, Sub, BitAnd/Or/Xor, Not, Neg, Shl, Shr
  - Conversions: from/to u16, i16, usize, Byte

- `Byte` - 8-bit byte with PDP-11 semantics
  - Same constant pattern as Word
  - Sign extension to Word
  - All bitwise operations

- `Register` - Register enumeration
  - R0-R7, SP, PC
  - to_code() / from_code() for instruction encoding
  - parse() for assembly syntax ("R0", "SP", "%5", etc.)
  - Display trait for disassembly

**No Dependencies**: Pure Rust, no external crates

### pdp11-asm (Assembler Library)

**Purpose**: MACRO-11 compatible assembler

**Dependencies**:
- `pdp11-common` - Uses Word, Byte, Register

**Components**:

1. **Lexer** (`lexer.rs`)
   - Tokenizes MACRO-11 assembly source
   - Handles identifiers, numbers, strings, symbols
   - Supports octal (default) and decimal numbers
   - Comment handling (semicolon)
   - Tested with basic token recognition

2. **Parser** (`parser.rs`)
   - Parses tokens into AST
   - Statement types: Instruction, Directive, Label
   - Mnemonic recognition (all PDP-11 instructions)
   - Operand parsing (partial - needs completion)
   - Directive parsing (stub)

3. **Addressing Modes** (`addressing.rs`)
   - All 8 base modes + PC-relative forms
   - encode() method produces 6-bit mode|register field
   - needs_extra_word() for index/immediate/etc.
   - Uses Register from pdp11-common

4. **Encoder** (`encoder.rs`)
   - encode_instruction() produces Vec<Word>
   - Opcode tables for all instructions
   - Double/single/branch operand encoding
   - Returns Word type from pdp11-common
   - Branch offset calculation (stub)

5. **Expression Evaluator** (`expr.rs`)
   - Evaluates expressions to i32
   - Binary ops: Add, Sub, Mul, Div, And, Or, Xor
   - Unary ops: Negate, Complement
   - Symbol resolution
   - Current location counter (.)

6. **Symbol Table** (`symbol.rs`)
   - HashMap-based symbol storage
   - define() and get() methods
   - Used for labels and constants

7. **Types** (`lib.rs`)
   - Instruction, Directive, Statement enums
   - Mnemonic enum (all PDP-11 instructions)
   - Operand, Expr types
   - BinaryOp, UnaryOp enums

**Supported Instructions**:
- Double operand: MOV, CMP, BIT, BIC, BIS, ADD, SUB (+ byte variants)
- Single operand: CLR, COM, INC, DEC, NEG, TST, ROR, ROL, ASR, ASL (+ byte)
- Branch: BR, BNE, BEQ, BGE, BLT, BGT, BLE, BPL, BMI, BHI, BLOS, BVC, BVS, BCC, BCS
- Subroutine: JSR, RTS
- EIS: MUL, DIV, ASH, ASHC, XOR, SOB
- Control: HALT, WAIT, RTI, IOT, RESET, NOP
- Flags: CLC, SEC, CLV, SEV, CLZ, SEZ, CLN, SEN, CCC, SCC

**Planned Directives**:
- .WORD, .BYTE - Data definition
- .ASCII, .ASCIZ - String data
- .BLKW, .BLKB - Block allocation
- .EVEN, .ODD - Alignment
- . = expr - Set location counter
- .TITLE, .SBTTL - Listing control
- .END - End of source

### pdp11-asm-cli (CLI Tool)

**Purpose**: Command-line assembler

**Dependencies**:
- `pdp11-asm` - Assembler library
- `pdp11-common` - Common types

**Features** (planned):
- Read .s assembly source files
- Output binary (a.out format)
- Listing generation (--list)
- Symbol table output (--symbols)
- Error reporting with line numbers

## Reusable Components from pdp11-core

### Currently Used

1. **Word and Byte types** - Copied to pdp11-common
   - Full implementation with all operations
   - Constants (ZERO, ONE, TWO, etc.)
   - Conversions and arithmetic

2. **Register enum** - Copied to pdp11-common
   - Added parse() method for assembly
   - Added to_code() / from_code() for encoding

### Not Yet Used (Emulator-Specific)

1. **Operand struct** - Emulator's runtime operand representation
   - Could be shared for disassembler
   - Assembler uses different AST representation

2. **Instruction enum** - Emulator's decoded instruction
   - Different from assembler's AST
   - Potential for shared opcode tables

3. **PSW (Processor Status Word)** - Runtime state
   - Not needed by assembler

4. **Memory structures** (Ram, Address, MmioSpace)
   - Emulator-specific
   - Assembler outputs raw bytes

5. **Device drivers** (Console, RK, KW11)
   - Emulator-only

## Next Steps

### Immediate (To Complete Basic Assembler)

1. Complete parser operand parsing
   - All addressing modes
   - Expression parsing in operands
   - Index values, immediate values

2. Implement two-pass assembly
   - Pass 1: Build symbol table, calculate locations
   - Pass 2: Resolve symbols, generate code

3. Complete encoder
   - Branch offset calculation (PC-relative)
   - Expression evaluation for immediate/index
   - Multi-word instruction generation

4. Add directive support
   - .WORD, .BYTE implementation
   - .ASCII/.ASCIZ string handling
   - .BLKW/.BLKB allocation
   - Location counter manipulation

5. Object file output
   - Binary format (raw bytes)
   - Or a.out format for compatibility

### Future Enhancements

1. Macro expansion
   - .MACRO/.ENDM definitions
   - Parameter substitution
   - Nested macros

2. Local labels
   - n$: style labels
   - Scope within macros

3. Listing generation
   - Source + object code
   - Symbol table
   - Cross-reference

4. Advanced directives
   - .INCLUDE for multi-file assembly
   - .IF/.ENDIF conditional assembly
   - .REPT/.ENDR repetition

5. Error recovery
   - Continue after errors
   - Multiple error reporting

6. Optimization
   - Branch distance optimization
   - Addressing mode selection

## Migration Path for pdp11-core

To make pdp11-core use pdp11-common:

1. Update pdp11-core/Cargo.toml:
   ```toml
   [dependencies]
   pdp11-common = { path = "../pdp11-common" }
   ```

2. In pdp11-core/src/lib.rs:
   ```rust
   pub use pdp11_common::{Word, Byte, Register};
   ```

3. Remove duplicate Word, Byte, Register implementations from pdp11-core

4. Update imports throughout pdp11-core to use re-exported types

5. Adjust any emulator-specific extensions (like MemoryAcceess trait)

This maintains backward compatibility while enabling code sharing.

## Testing Strategy

### Unit Tests

- lexer.rs: Token recognition tests ✓ (basic)
- register.rs: Parsing and encoding tests ✓
- types.rs: Word/Byte operations (inherited from pdp11-core)
- encoder.rs: Instruction encoding tests (needed)
- expr.rs: Expression evaluation tests (needed)

### Integration Tests

- Complete instruction assembly (needed)
- Multi-pass assembly with symbols (needed)
- Directive processing (needed)

### Compatibility Tests

- Compare output with DEC MACRO-11 (future)
- Assemble test programs and run in emulator (future)

## Documentation

- [x] README.md with architecture overview
- [x] Code comments in all modules
- [x] This summary document
- [ ] API documentation (rustdoc)
- [ ] User manual
- [ ] MACRO-11 syntax guide

## Build Status

All crates added to workspace in Cargo.toml:
- pdp11-common
- pdp11-asm
- pdp11-asm-cli

Note: Need to verify build with `cargo check --workspace`
