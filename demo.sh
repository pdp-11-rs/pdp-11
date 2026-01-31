#!/bin/bash
# Demo script for PDP-11 emulator interactive mode
#
# The emulator will boot to the ODT (@) prompt.
# You can then type commands:
#   G - Go (start execution at current PC)
#   1000G - Go starting at address 1000
#   <addr>/ - Examine memory at address
#   Ctrl-C - Exit

echo "PDP-11 Emulator - Interactive Demo"
echo "==================================="
echo ""
echo "Booting to ODT prompt..."
echo "When you see '@', you can type ODT commands."
echo "Press Ctrl-C to exit."
echo ""

# Run with minimal logging to not interfere with console I/O
RUST_LOG=error cargo run --release

# Restore terminal on exit
stty sane
