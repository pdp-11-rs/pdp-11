# Interactive Console Testing

The PDP-11 emulator now supports interactive keyboard input!

## Quick Start

```bash
# Run the emulator
cargo run --release

# Or use the demo script
./demo.sh
```

## Disk Image

The current `rk0.img` is a minimal bootable test image. For a full RT-11 operating system experience, see [RT11-SETUP.md](RT11-SETUP.md) for instructions on obtaining and installing RT-11.

## What to Expect

1. The emulator boots and displays the `@` prompt (ODT - Octal Debugging Technique)
2. You can now type commands directly (no need to press Enter first)
3. Try typing `G` to execute the "Go" command
4. The bootloader will echo your input and respond

## ODT Commands (Basic)

- `G` - Go (start execution at current PC)
- `<addr>G` - Go starting at octal address (e.g., `1000G`)
- `<addr>/` - Examine memory at address
- Ctrl-C - Exit the emulator

## Terminal Behavior

- **Raw mode**: Characters are sent immediately as you type
- **No echo**: The bootloader itself echoes characters (you'll see lowercase responses)
- **Auto-restore**: Terminal settings are restored automatically on exit

## Platform Support

Currently works on Unix-like systems (Linux, macOS). Windows support requires additional implementation.

## Technical Details

The console device now:
- Reads from stdin in non-blocking mode
- Sets the READER_DONE bit when a character is available
- Properly integrates with the MMIO system for device register access

The terminal is set to raw mode which:
- Disables line buffering (canonical mode)
- Disables echo (characters aren't printed twice)
- Allows immediate character-by-character input
