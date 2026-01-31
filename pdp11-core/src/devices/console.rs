use crate::devices::*;
use std::io::{self, Write};

/// DL11/KL11 Console Serial Interface
///
/// Provides character I/O via stdin/stdout with memory-mapped registers
/// Operating in polling mode (no interrupts initially)
pub struct Console {
    // Receiver status/buffer
    rcsr: Word,
    rbuf: Word,
    // Transmitter status/buffer
    xcsr: Word,
    xbuf: Word,
}

// Console register addresses
pub const RCSR: Address<Word> = Address::from_u16(0o177560);
pub const RBUF: Address<Word> = Address::from_u16(0o177562);
pub const XCSR: Address<Word> = Address::from_u16(0o177564);
pub const XBUF: Address<Word> = Address::from_u16(0o177566);

// Status register bits
const READER_ENABLE: Word = Word::from_u16(0o000001); // Receiver enable
const READER_DONE: Word = Word::from_u16(0o000200); // Receiver done (data available)
const XMIT_READY: Word = Word::from_u16(0o000200); // Transmitter ready

impl Console {
    /// Create a new console with default state
    pub fn new() -> Self {
        Self {
            rcsr: READER_ENABLE,
            rbuf: Word::zero(),
            xcsr: XMIT_READY, // Transmitter always ready initially
            xbuf: Word::zero(),
        }
    }

    /// Initialize console registers in RAM
    pub fn init_registers(&self, ram: &mut Ram) {
        ram.write_direct(RCSR, self.rcsr);
        ram.write_direct(RBUF, self.rbuf);
        ram.write_direct(XCSR, self.xcsr);
        ram.write_direct(XBUF, self.xbuf);
    }

    /// Read a console register value
    pub fn read_register(&mut self, address: Address<Word>) -> Word {
        match address {
            RCSR => {
                // Check if there's input available (non-blocking)
                self.check_input();
                self.rcsr
            }
            RBUF => {
                // Reading RBUF clears the DONE bit
                let data = self.rbuf;
                self.rcsr = self.rcsr & !READER_DONE;
                data
            }
            XCSR => self.xcsr,
            XBUF => self.xbuf, // Write-only, but return current value
            _ => Word::zero(),
        }
    }

    /// Write to a console register
    pub fn write_register(&mut self, address: Address<Word>, value: Word) {
        match address {
            RCSR => {
                // Only certain bits are writable (enable bit)
                self.rcsr = (self.rcsr & !READER_ENABLE) | (value & READER_ENABLE);
            }
            RBUF => {
                // RBUF is read-only, ignore writes
            }
            XCSR => {
                // XCSR bits (typically only interrupt enable is writable)
                self.xcsr = value;
            }
            XBUF => {
                // Write character to output
                self.output_char(value);
                // Transmitter remains ready
                self.xcsr = XMIT_READY;
            }
            _ => {}
        }
    }

    /// Check for input character (non-blocking)
    fn check_input(&mut self) {
        // Use stdin in non-blocking mode
        // For now, we'll skip non-blocking input as it requires platform-specific code
        // In a real implementation, this would check if stdin has data available
    }

    /// Output a character to stdout
    fn output_char(&mut self, value: Word) {
        let ch = (value.as_u16() & 0o377) as u8; // Low byte only
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.write_all(&[ch]);
        let _ = handle.flush();
    }

    /// Manually input a character (for testing or when input is available)
    #[cfg(test)]
    pub fn input_char(&mut self, ch: u8) {
        self.rbuf = Word::from(ch as u16);
        self.rcsr = self.rcsr | READER_DONE;
    }
}

impl fmt::Debug for Console {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Console")
            .field("rcsr", &format!("{:#08o}", self.rcsr.as_u16()))
            .field("rbuf", &format!("{:#08o}", self.rbuf.as_u16()))
            .field("xcsr", &format!("{:#08o}", self.xcsr.as_u16()))
            .field("xbuf", &format!("{:#08o}", self.xbuf.as_u16()))
            .finish()
    }
}

impl MmioDevice for Console {
    fn read_word(&mut self, address: Address<Word>) -> Word {
        self.read_register(address)
    }

    fn write_word(&mut self, address: Address<Word>, value: Word) {
        self.write_register(address, value);
    }

    fn address_range(&self) -> (u16, u16) {
        (0o177560, 0o177566)
    }

    fn handles_word_address(&self, address: Address<Word>) -> bool {
        matches!(address, RCSR | RBUF | XCSR | XBUF)
    }

    fn handles_byte_address(&self, address: Address<Byte>) -> bool {
        // Console registers are word-aligned, check if byte address falls within range
        let start = Address::<Byte>::from_u16(0o177560);
        let end = Address::<Byte>::from_u16(0o177567); // XBUF + 1
        address >= start && address <= end
    }
}
