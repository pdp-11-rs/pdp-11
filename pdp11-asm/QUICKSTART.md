## PDP-11 MACRO-11 Assembler - Quick Reference

### Project Structure

```
┌─────────────────────────────────────────────────────┐
│                   Workspace Root                    │
└─────────────────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
    ┌───▼────┐      ┌────▼───┐      ┌────▼────┐
    │ COMMON │      │  CORE  │      │   ASM   │
    │        │      │        │      │         │
    │ Word   │◄─────┤Emulator│      │Assembler│◄────┐
    │ Byte   │      │        │      │         │     │
    │Register│◄─────────────────────┤         │     │
    └────────┘      └────────┘      └────┬────┘     │
                                         │          │
                                    ┌────▼──────┐   │
                                    │  ASM-CLI  │   │
                                    │           │   │
                                    │pdp11-asm  │───┘
                                    └───────────┘
```

### Crate Dependencies

```
pdp11-common:          (no dependencies)
  ├─ Word, Byte, Register types
  ├─ Constants (ZERO, ONE, TWO, etc.)
  └─ Arithmetic/bitwise operations

pdp11-core:            (uses pdp11-common - future)
  ├─ CPU emulator
  ├─ Memory management
  ├─ Device drivers
  └─ Instruction execution

pdp11-asm:             depends on: pdp11-common
  ├─ Lexer (tokenization)
  ├─ Parser (AST generation)
  ├─ Encoder (instruction → binary)
  ├─ Symbol table
  ├─ Expression evaluator
  └─ Addressing modes

pdp11-asm-cli:         depends on: pdp11-asm, pdp11-common
  └─ Command-line assembler tool
```

### Code Sharing

**What's Shared (pdp11-common):**
- ✅ Word type (16-bit with PDP-11 semantics)
- ✅ Byte type (8-bit with PDP-11 semantics)
- ✅ Register enum (R0-R7, SP, PC)
- ✅ Type conversions and operations
- ✅ Constants (ZERO, ONE, TWO, etc.)

**What's Unique to Assembler (pdp11-asm):**
- Lexer and Parser (source → AST)
- Instruction encoder (AST → binary)
- Symbol table and expressions
- Directive processing
- Two-pass assembly logic

**What's Unique to Emulator (pdp11-core):**
- CPU execution engine
- Memory management (RAM, MMIO)
- Device drivers (Console, RK, KW11)
- Instruction decoder
- PSW and interrupt handling

### Building

```bash
# Build entire workspace
cargo build --workspace

# Build specific crate
cargo build -p pdp11-common
cargo build -p pdp11-asm
cargo build -p pdp11-asm-cli

# Run tests
cargo test -p pdp11-common
cargo test -p pdp11-asm

# Run assembler example
cargo run --package pdp11-asm --example test_basic

# Run assembler CLI
cargo run --bin pdp11-asm -- input.s
```

### Implementation Status

**pdp11-common (Complete):**
- ✅ Word type with all operations
- ✅ Byte type with all operations
- ✅ Register enum with parsing
- ✅ Full test coverage (from pdp11-core)

**pdp11-asm (Partial):**
- ✅ Lexer (tokenization)
- ✅ Symbol table
- ✅ Expression evaluator
- ✅ Addressing mode encoding
- ✅ Basic instruction encoding
- ⚠️  Parser (mnemonic recognition done, operand parsing needed)
- ⚠️  Encoder (opcodes done, multi-word instructions needed)
- ❌ Directive support (stubs only)
- ❌ Two-pass assembly
- ❌ Listing generation

**pdp11-asm-cli (Minimal):**
- ✅ Basic CLI structure
- ❌ Full assembly pipeline
- ❌ File I/O
- ❌ Error reporting

### Next Development Steps

1. **Complete Parser:**
   - Operand parsing for all addressing modes
   - Expression parsing in operands
   - Directive argument parsing

2. **Two-Pass Assembly:**
   - Pass 1: Build symbol table, calculate sizes
   - Pass 2: Resolve symbols, generate code

3. **Complete Encoder:**
   - Branch offset calculation
   - Index/immediate value encoding
   - Multi-word instruction generation

4. **Add Directives:**
   - .WORD, .BYTE data definition
   - .ASCII, .ASCIZ strings
   - .BLKW, .BLKB allocation
   - Location counter manipulation

5. **Object File Output:**
   - Binary format or
   - a.out format for compatibility

### Example Usage (Planned)

```asm
; test.s - Simple test program
        .TITLE  Test Program

START:  MOV     #100, R0        ; Load immediate
        MOV     R0, RESULT      ; Store to memory
LOOP:   DEC     R0              ; Decrement
        BNE     LOOP            ; Loop if not zero
        HALT

RESULT: .WORD   0               ; Storage

        .END    START
```

```bash
# Assemble
$ pdp11-asm test.s -o test.bin

# With listing
$ pdp11-asm test.s -o test.bin --list test.lst

# Show symbols
$ pdp11-asm test.s --symbols
START   000000
LOOP    000006
RESULT  000012
```

### Files Created

```
NEW FILES:
  pdp11-common/
    Cargo.toml
    src/lib.rs
    src/types.rs
    src/register.rs

  pdp11-asm/
    Cargo.toml
    README.md
    src/lib.rs
    src/addressing.rs
    src/encoder.rs
    src/error.rs
    src/expr.rs
    src/lexer.rs
    src/parser.rs
    src/symbol.rs
    examples/test_basic.rs

  pdp11-asm-cli/
    Cargo.toml
    src/main.rs

  ASSEMBLER.md (this file)

MODIFIED FILES:
  Cargo.toml (workspace members)
```
