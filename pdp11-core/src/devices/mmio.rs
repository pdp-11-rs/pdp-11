use super::*;

/// Memory-Mapped I/O device trait
///
/// Peripherals implement this trait to handle reads and writes
/// to their memory-mapped registers.
#[allow(dead_code)]
pub trait MmioDevice {
    /// Read a word from a device register
    fn read_word(&mut self, address: Address<Word>) -> Word;

    /// Write a word to a device register
    fn write_word(&mut self, address: Address<Word>, value: Word);

    /// Read a byte from a device register
    fn read_byte(&mut self, address: Address<Byte>) -> Byte {
        // Default implementation: read word and extract appropriate byte
        let word_addr = Address::from_u16(address.as_u16() & !1);
        let word = self.read_word(word_addr);
        if address.as_u16() & 1 == 0 {
            // Even address: low byte
            Byte::from(word.as_u16() as u8)
        } else {
            // Odd address: high byte
            Byte::from((word.as_u16() >> 8) as u8)
        }
    }

    /// Write a byte to a device register
    fn write_byte(&mut self, address: Address<Byte>, value: Byte) {
        // Default implementation: read word, modify byte, write back
        let word_addr = Address::from_u16(address.as_u16() & !1);
        let mut word = self.read_word(word_addr);
        if address.as_u16() & 1 == 0 {
            // Even address: low byte
            let high = word.as_u16() & 0xFF00;
            word = Word::from_u16(high | value.as_u16());
        } else {
            // Odd address: high byte
            let low = word.as_u16() & 0x00FF;
            word = Word::from_u16(low | (value.as_u16() << 8));
        }
        self.write_word(word_addr, word);
    }

    /// Get the address range for this device
    /// Returns (start_address, end_address) as u16 values, inclusive
    fn address_range(&self) -> (u16, u16);

    /// Check if this device handles the given word address
    fn handles_word_address(&self, address: Address<Word>) -> bool {
        // We need to extract the u16 value from the Address
        // Since Address wraps a Word and Word has as_u16(), we need
        // a way to get that value. For now, use from_u16 to create
        // addresses and compare directly.
        let (start, end) = self.address_range();
        let start_addr = Address::<Word>::from_u16(start);
        let end_addr = Address::<Word>::from_u16(end);
        address >= start_addr && address <= end_addr
    }

    /// Check if this device handles the given byte address
    fn handles_byte_address(&self, address: Address<Byte>) -> bool {
        let (start, end) = self.address_range();
        let start_addr = Address::<Byte>::from_u16(start);
        let end_addr = Address::<Byte>::from_u16(end);
        address >= start_addr && address <= end_addr
    }
}

/// MMIO address space manager
///
/// The PDP-11 typically uses the upper 4KB (0o160000-0o177777)
/// for memory-mapped I/O devices.
#[derive(Debug)]
#[allow(dead_code)]
pub struct MmioSpace {
    /// Standard I/O page start address
    pub io_page_start: u16,
}

#[allow(dead_code)]
impl MmioSpace {
    /// Create a new MMIO space with standard I/O page
    pub fn new() -> Self {
        Self {
            io_page_start: 0o160000, // Standard PDP-11 I/O page
        }
    }

    /// Check if an address is in the I/O page
    pub fn is_io_space(&self, address: u16) -> bool {
        address >= self.io_page_start
    }

    /// Check if a word address is in the I/O page
    pub fn is_io_space_word(&self, address: Address<Word>) -> bool {
        self.is_io_space(address.as_u16())
    }

    /// Check if a byte address is in the I/O page
    pub fn is_io_space_byte(&self, address: Address<Byte>) -> bool {
        self.is_io_space(address.as_u16())
    }
}

impl Default for MmioSpace {
    fn default() -> Self {
        Self::new()
    }
}
