# RT-11 Disk Image Setup

This guide helps you obtain and set up a proper RT-11 operating system disk image for the PDP-11 emulator.

## Quick Start

The current `rk0.img` contains a minimal bootable stub. For a full RT-11 operating system, you need to obtain a proper disk image.

## Option 1: Download Pre-made RT-11 RK05 Image

### From Internet Archive
```bash
# RT-11 v5.3 RK05 disk image
curl -L -o rt11v53.rk05 "https://archive.org/download/rt-11_v5.3_rk05/rt11_v5.3.rk05"

# Verify size (should be 2,494,464 bytes for RK05)
ls -l rt11v53.rk05

# Use it
cp rt11v53.rk05 rk0.img
```

### From SIMH Project
Visit https://github.com/simh/simh and look in the `BIN/` directory for RT-11 disk images.

## Option 2: Build from RT-11 Distribution

If you have access to RT-11 distribution kits:

```bash
# 1. Get RT-11 v5.3 distribution
# 2. Use SIMH PDP-11 simulator to install RT-11 to an RK05 image
# 3. Copy the resulting .dsk file to rk0.img
```

## Option 3: Use the Minimal Test Image (Current)

The current `rk0.img` is a minimal bootable image that:
- ✅ Boots to ODT prompt
- ✅ Tests emulator functionality
- ❌ Does NOT include RT-11 operating system
- ❌ Does NOT have a file system

It's suitable for:
- Testing the emulator
- Running simple programs loaded via ODT
- Development and debugging

## RK05 Disk Geometry

The emulator expects RK05 disk images with these specifications:
- **Size:** 2,494,464 bytes (exact)
- **Cylinders:** 203
- **Surfaces:** 2 (heads)
- **Sectors per track:** 12
- **Bytes per sector:** 512

Formula: `203 × 2 × 12 × 512 = 2,494,464 bytes`

The emulator will warn if your disk image size doesn't match.

## Verifying Your Disk Image

```bash
# Check size
ls -l rk0.img

# Should output:
# -rw-r--r--  1 user  staff  2494464 ... rk0.img

# Run the emulator
cargo run

# You should see:
# - No RK05 size warnings (if size is correct)
# - Boot sequence
# - Either @ prompt (ODT) or RT-11 prompt (.)
```

## Expected Boot Behavior

### With Minimal Test Image (Current):
```
@
```
You get the ODT (Octal Debugging Tool) prompt where you can examine memory and run code.

### With Full RT-11:
```
RT-11SJ  V05.03

.TYPE V5NOTE.TXT

.
```
You get the RT-11 monitor prompt (`.`) where you can use RT-11 commands like `DIR`, `TYPE`, `RUN`, etc.

## RT-11 Resources

- **SIMH Project:** https://github.com/simh/simh
- **Bitsavers:** http://bitsavers.org/bits/DEC/pdp11/rt-11/
- **Archive.org:** Search for "RT-11 PDP-11"
- **DECUS:** Historical DEC User Society archives

## Legal Note

RT-11 is legacy software. While DEC (now part of HP) no longer actively sells or supports RT-11, the copyright may still apply. Check current licensing terms before using in production environments. Many hobbyist uses are generally accepted by the community.

## Need Help?

If you have trouble obtaining RT-11 images:
1. Check if your institution has DEC software archives
2. Look for "hobbyist license" programs
3. Use SIMH's included images (often freely distributable for hobbyist use)
4. Contact the SIMH community for guidance

## Testing Without RT-11

The emulator works fine with the current minimal image for:
- Testing PDP-11 instruction set
- Debugging emulator features
- Learning PDP-11 assembly
- Running small test programs via ODT

You don't NEED RT-11 to use the emulator, but it provides a much richer experience!
