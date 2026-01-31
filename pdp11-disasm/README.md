# pdp11-disasm

PDP-11 Disassembler - Convert machine code to MACRO-11 assembly language.

## Usage

```bash
# Disassemble a binary file
pdp11-disasm program.bin

# Show addresses and opcodes
pdp11-disasm -a -o bootrom.o

# Disassemble from stdin
cat program.bin | pdp11-disasm

# Disassemble starting at specific address (octal)
pdp11-disasm -s 2000 program.bin

# Disassemble only N words
pdp11-disasm -n 20 program.bin

# Combine options
pdp11-disasm -a -o -s 2000 -n 50 disk.img
```

## Options

- `-a, --show-addresses` - Show memory addresses
- `-o, --show-opcodes` - Show instruction opcodes (octal)
- `-s, --start <ADDR>` - Start address for disassembly (octal, e.g., 2000)
- `-n, --count <COUNT>` - Number of words to disassemble
- `<FILE>` - Input file (omit to read from stdin)

## Examples

### Disassemble bootrom

```bash
$ pdp11-disasm -a -o bootrom.o
000000:  042113  BIC    (R1)+, (R3)
000002:  012706  MOV    (PC)+, SP
000004:  002000  BGE    .+0
000006:  012700  MOV    (PC)+, R0
000010:  000000  HALT
000012:  010003  MOV    R0, R3
000014:  000303  SWAB   R3
000016:  006303  ASL    R3
```

### Analyze disk image

```bash
$ pdp11-disasm -a -n 10 -s 1000 disk.img
001000:  012706  MOV    (PC)+, SP
001002:  002000  BGE    .+0
001004:  012700  MOV    (PC)+, R0
```

## Features

- Disassembles all PDP-11 instructions
- Handles all addressing modes
- Supports both word and byte instructions
- Octal formatting (standard for PDP-11)
- Clean MACRO-11 syntax output

## Note

Extension words for indexed/immediate addressing modes are currently shown as "Invalid opcode" since they are data, not instructions. This is expected behavior.
