use crate::devices::*;
use std::io::{self, Write};

/// DL11/KL11 Console Serial Interface
///
/// Provides character I/O via stdin/stdout with memory-mapped registers
///
/// **Specifications:**
/// - Receiver Interrupt Vector: 0o060
/// - Transmitter Interrupt Vector: 0o064
/// - Interrupt Priority: 4
/// - Registers at 0o177560-0o177566
///
/// **RCSR (Receiver Control/Status) bits:**
/// - Bit 7: Reader Done (DONE) - set when character available
/// - Bit 6: Interrupt Enable (IE) - enables receiver interrupts
/// - Bit 0: Reader Enable (RE) - enables receiver
///
/// **XCSR (Transmitter Control/Status) bits:**
/// - Bit 7: Transmitter Ready (READY) - ready to send
/// - Bit 6: Interrupt Enable (IE) - enables transmitter interrupts
pub struct Console {
    // Receiver status/buffer
    rcsr: Word,
    rbuf: Word,
    // Transmitter status/buffer
    xcsr: Word,
    xbuf: Word,
    // Interrupt state
    rx_interrupt_pending: bool,
    tx_interrupt_pending: bool,
}

// Console register addresses
pub const RCSR: Address<Word> = Address::from_u16(0o177560);
pub const RBUF: Address<Word> = Address::from_u16(0o177562);
pub const XCSR: Address<Word> = Address::from_u16(0o177564);
pub const XBUF: Address<Word> = Address::from_u16(0o177566);

// Status register bits
const READER_ENABLE: Word = Word::from_u16(0o000001); // Receiver enable (bit 0)
const READER_IE: Word = Word::from_u16(0o000100); // Receiver interrupt enable (bit 6)
const READER_DONE: Word = Word::from_u16(0o000200); // Receiver done/data available (bit 7)
const XMIT_IE: Word = Word::from_u16(0o000100); // Transmitter interrupt enable (bit 6)
const XMIT_READY: Word = Word::from_u16(0o000200); // Transmitter ready (bit 7)

// Interrupt vectors and priority
pub const CONSOLE_RX_VECTOR: u16 = 0o060;
pub const CONSOLE_TX_VECTOR: u16 = 0o064;
pub const CONSOLE_PRIORITY: u8 = 4;

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    /// Create a new console with default state
    pub fn new() -> Self {
        Self {
            rcsr: READER_ENABLE,
            rbuf: Word::zero(),
            xcsr: XMIT_READY, // Transmitter always ready initially
            xbuf: Word::zero(),
            rx_interrupt_pending: false,
            tx_interrupt_pending: false,
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
                // Reading RBUF clears the DONE bit and interrupt
                let data = self.rbuf;
                self.rcsr &= !READER_DONE;
                self.rx_interrupt_pending = false;
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
                // Only enable and IE bits are writable
                self.rcsr = (self.rcsr & !(READER_ENABLE | READER_IE))
                    | (value & (READER_ENABLE | READER_IE));
                // Check if we should generate an interrupt
                self.update_rx_interrupt();
            }
            RBUF => {
                // RBUF is read-only, ignore writes
            }
            XCSR => {
                // Only IE bit is writable for XCSR
                self.xcsr = (self.xcsr & !XMIT_IE) | (value & XMIT_IE);
                // Check if we should generate an interrupt
                self.update_tx_interrupt();
            }
            XBUF => {
                // Write character to output
                self.output_char(value);
                // Transmitter remains ready (always ready for next character)
                self.xcsr = (self.xcsr & XMIT_IE) | XMIT_READY;
                self.update_tx_interrupt();
            }
            _ => {}
        }
    }

    /// Check for input character (non-blocking)
    fn check_input(&mut self) {
        // Try to read one byte from stdin without blocking
        #[cfg(unix)]
        {
            self.check_input_unix();
        }

        #[cfg(not(unix))]
        {
            // Non-Unix platforms: input not supported yet
            // Could use Windows-specific APIs or a thread-based approach
        }
    }

    #[cfg(unix)]
    fn check_input_unix(&mut self) {
        use std::io::Read;

        // Use termion's async stdin for non-blocking input
        let mut async_stdin = termion::async_stdin();

        // Try to read one byte
        let mut buf = [0u8; 1];
        if let Ok(1) = async_stdin.read(&mut buf) {
            // Got a character
            self.rbuf = Word::from(buf[0] as u16);
            self.rcsr |= READER_DONE;
            self.update_rx_interrupt();
        }
        // No data available is normal for non-blocking, just return
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
    pub fn input_char(&mut self, ch: u8) {
        self.rbuf = Word::from(ch as u16);
        self.rcsr |= READER_DONE;
        self.update_rx_interrupt();
    }

    /// Update receiver interrupt state based on DONE and IE bits
    fn update_rx_interrupt(&mut self) {
        // Interrupt pending if DONE is set and IE is enabled
        self.rx_interrupt_pending =
            (self.rcsr & READER_DONE) != Word::zero() && (self.rcsr & READER_IE) != Word::zero();
    }

    /// Update transmitter interrupt state based on READY and IE bits
    fn update_tx_interrupt(&mut self) {
        // Interrupt pending if READY is set and IE is enabled
        self.tx_interrupt_pending =
            (self.xcsr & XMIT_READY) != Word::zero() && (self.xcsr & XMIT_IE) != Word::zero();
    }

    /// Check if receiver interrupt is pending
    pub fn rx_interrupt_pending(&self) -> bool {
        self.rx_interrupt_pending
    }

    /// Check if transmitter interrupt is pending
    pub fn tx_interrupt_pending(&self) -> bool {
        self.tx_interrupt_pending
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
