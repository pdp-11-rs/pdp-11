# PDP-11 MACRO-11 Assembler

A MACRO-11 compatible assembler for the PDP-11 architecture, written in Rust.

## Architecture

The project is organized into multiple crates for code reuse:

```
pdp-11/
├── pdp11-common/      # Shared types (Word, Byte, Register)
├── pdp11-core/        # Emulator (uses pdp11-common)
├── pdp11-asm/         # Assembler library (uses pdp11-common)
├── pdp11-asm-cli/     # Assembler CLI tool
└── pdp11/             # Emulator binary
```

### Code Reuse Strategy

**pdp11-common** provides shared primitives:
- `Word` - 16-bit word type with PDP-11 semantics
- `Byte` - 8-bit byte type with PDP-11 semantics  
- `Register` - Register enum (R0-R7, SP, PC)
- Constants: ZERO, ONE, TWO, MAX_POSITIVE, etc.
- Arithmetic and bitwise operations

**pdp11-asm** (assembler library) uses:
- `Word` and `Byte` for encoding instructions
- `Register` for register operands and parsing
- Provides MACRO-11 compatible assembly

**pdp11-core** (emulator) uses:
- `Word` and `Byte` for memory and operations
- `Register` for CPU register file
- Extended with emulator-specific types (Address, Operand, etc.)

## Features

### Implemented

- ✅ Lexer with token recognition
- ✅ Symbol table
- ✅ Expression evaluator (arithmetic, logical)
- ✅ Basic instruction encoding
- ✅ All PDP-11 addressing modes
- ✅ Register parsing

### In Progress

- 🔨 Complete parser (operands, directives)
- 🔨 Full instruction encoder
- 🔨 Two-pass assembly
- 🔨 Listing generation

### Planned

- 📋 Directive support (.WORD, .BYTE, .ASCII, .BLKW, etc.)
- 📋 Macro expansion
- 📋 Local labels
- 📋 Object file output (a.out format)
- 📋 Error reporting with line numbers
- 📋 Cross-reference listing

## Supported Instructions

All PDP-11 instructions are recognized:

### Double Operand
- MOV, MOVB - Move
- CMP, CMPB - Compare
- BIT, BITB - Bit test
- BIC, BICB - Bit clear
- BIS, BISB - Bit set
- ADD - Add
- SUB - Subtract

### Single Operand
- CLR, CLRB - Clear
- COM, COMB - Complement
- INC, INCB - Increment
- DEC, DECB - Decrement
- NEG, NEGB - Negate
- ADC, ADCB - Add carry
- SBC, SBCB - Subtract carry
- TST, TSTB - Test
- ROR, RORB - Rotate right
- ROL, ROLB - Rotate left
- ASR, ASRB - Arithmetic shift right
- ASL, ASLB - Arithmetic shift left
- JMP - Jump
- SWAB - Swap bytes

### Branch
- BR - Branch (unconditional)
- BNE, BEQ - Branch on not equal/equal
- BGE, BLT, BGT, BLE - Signed comparisons
- BPL, BMI - Branch on plus/minus
- BHI, BLOS - Unsigned comparisons
- BVC, BVS - Branch on overflow
- BCC, BCS - Branch on carry

### Subroutine
- JSR - Jump to subroutine
- RTS - Return from subroutine

### EIS (Extended Instruction Set)
- MUL - Multiply
- DIV - Divide
- ASH - Arithmetic shift
- ASHC - Arithmetic shift combined
- XOR - Exclusive OR
- SOB - Subtract one and branch

### Control
- HALT, WAIT, RTI, IOT, RESET, NOP
- CLC, SEC, CLV, SEV, CLZ, SEZ, CLN, SEN
- CCC, SCC

## Addressing Modes

All 8 PDP-11 addressing modes plus PC-relative forms:

- **Register**: R0
- **Register Deferred**: (R0)
- **Autoincrement**: (R0)+
- **Autoincrement Deferred**: @(R0)+
- **Autodecrement**: -(R0)
- **Autodecrement Deferred**: @-(R0)
- **Index**: X(R0)
- **Index Deferred**: @X(R0)
- **Immediate**: #value
- **Absolute**: @#address
- **Relative**: label
- **Relative Deferred**: @label

## Usage

```bash
# Assemble a file
cargo run --bin pdp11-asm -- input.s -o output.bin

# Show assembly listing
cargo run --bin pdp11-asm -- input.s --list

# Generate symbol table
cargo run --bin pdp11-asm -- input.s --symbols
```

## Example Assembly

```asm
; Simple program
        .TITLE  Hello World

START:  MOV     #MSG, R0        ; Load message address
        MOV     #1, R1          ; Character count
LOOP:   MOVB    (R0)+, R2       ; Get character
        BEQ     DONE            ; Exit if null
        ; Output character (would call OS)
        INC     R1              ; Count characters
        BR      LOOP            ; Continue

DONE:   HALT

MSG:    .ASCII  "Hello, PDP-11!"<0>

        .END    START
```

## Development

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Check assembler
cargo check -p pdp11-asm

# Build common library
cargo build -p pdp11-common
```

## Testing

```bash
# Test lexer
cargo test -p pdp11-asm lexer

# Test encoder
cargo test -p pdp11-asm encoder

# Test common types
cargo test -p pdp11-common
```

## Design Goals

1. **MACRO-11 Compatibility**: Support standard DEC MACRO-11 assembly syntax
2. **Code Reuse**: Share types between assembler and emulator
3. **Correctness**: Produce binary-compatible output with DEC assemblers
4. **Performance**: Fast assembly for large programs
5. **Error Messages**: Clear, helpful error reporting

## References

- DEC PDP-11 MACRO-11 Language Reference Manual
- PDP-11 Processor Handbook
- MACRO-11 Reference Card
