use crate::devices::*;

/// KW11-L Line Time Clock
///
/// Provides periodic interrupts at 60 Hz (or 50 Hz depending on power frequency)
/// Used for timekeeping and task scheduling
///
/// **Specifications:**
/// - Interrupt Vector: 0o100
/// - Interrupt Priority: 6
/// - Frequency: 60 Hz (line frequency in North America) or 50 Hz (Europe/Asia)
/// - Register: LKS (Line Clock Status) at 0o177546
///
/// **LKS Register bits:**
/// - Bit 7: Monitor (not implemented)
/// - Bit 6: Interrupt Enable (IE)
/// - Bit 5-0: Unused
///
/// **Operation:**
/// - When IE is set and clock ticks, generates interrupt at vector 0o100
/// - Software should service interrupt promptly to acknowledge
#[derive(Debug)]
pub struct Kw11 {
    lks: Word,       // Line Clock Status register
    tick_count: u64, // Tick counter (incremented each update)
    interrupt_pending: bool,
}

// KW11-L Register address
pub const LKS: Address<Word> = Address::from_u16(0o177546);

// LKS bits
const IE: Word = Word::from_u16(0o000100); // Interrupt Enable (bit 6)

// Interrupt vector and priority
pub const KW11_VECTOR: u16 = 0o100;
pub const KW11_PRIORITY: u8 = 6;

impl Kw11 {
    pub fn new() -> Self {
        Self {
            lks: Word::zero(),
            tick_count: 0,
            interrupt_pending: false,
        }
    }

    /// Simulate a clock tick (called periodically by CPU)
    /// Returns true if an interrupt should be generated
    pub fn tick(&mut self) -> bool {
        self.tick_count += 1;

        // Check if interrupts are enabled
        if (self.lks & IE) != Word::zero() {
            self.interrupt_pending = true;
            true
        } else {
            false
        }
    }

    /// Check if interrupt is pending
    pub fn interrupt_pending(&self) -> bool {
        self.interrupt_pending
    }

    /// Clear pending interrupt (called after interrupt is serviced)
    pub fn clear_interrupt(&mut self) {
        self.interrupt_pending = false;
    }

    /// Get tick count (for testing/debugging)
    #[cfg(test)]
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
}

impl Default for Kw11 {
    fn default() -> Self {
        Self::new()
    }
}

impl MmioDevice for Kw11 {
    fn read_word(&mut self, address: Address<Word>) -> Word {
        if address == LKS {
            self.lks
        } else {
            Word::zero()
        }
    }

    fn write_word(&mut self, address: Address<Word>, value: Word) {
        if address == LKS {
            self.lks = value;
            // If interrupts were disabled, clear any pending interrupt
            if (value & IE) == Word::zero() {
                self.interrupt_pending = false;
            }
        }
    }

    fn address_range(&self) -> (u16, u16) {
        (LKS.as_u16(), LKS.as_u16())
    }

    fn read_byte(&mut self, address: Address<Byte>) -> Byte {
        let word_addr = Address::<Word>::from_u16(address.as_u16() & !1);
        let word = self.read_word(word_addr);

        if address.as_u16() & 1 == 0 {
            Byte::from(word.as_u16() as u8)
        } else {
            Byte::from((word.as_u16() >> 8) as u8)
        }
    }

    fn write_byte(&mut self, address: Address<Byte>, value: Byte) {
        let word_addr = Address::<Word>::from_u16(address.as_u16() & !1);
        let mut word = self.read_word(word_addr);

        if address.as_u16() & 1 == 0 {
            word = Word::from_u16((word.as_u16() & 0xFF00) | value.as_u8() as u16);
        } else {
            word = Word::from_u16((word.as_u16() & 0x00FF) | ((value.as_u8() as u16) << 8));
        }

        self.write_word(word_addr, word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MmioDevice;

    #[test]
    fn test_kw11_new() {
        let kw11 = Kw11::new();
        assert_eq!(kw11.lks.as_u16(), 0);
        assert_eq!(kw11.tick_count, 0);
        assert!(!kw11.interrupt_pending());
    }

    #[test]
    fn test_kw11_tick_no_interrupt_when_disabled() {
        let mut kw11 = Kw11::new();

        // Tick without interrupt enable
        let interrupt = kw11.tick();

        assert!(!interrupt);
        assert!(!kw11.interrupt_pending());
        assert_eq!(kw11.tick_count(), 1);
    }

    #[test]
    fn test_kw11_tick_generates_interrupt_when_enabled() {
        let mut kw11 = Kw11::new();

        // Enable interrupts
        kw11.write_word(LKS, IE);

        // Tick should generate interrupt
        let interrupt = kw11.tick();

        assert!(interrupt);
        assert!(kw11.interrupt_pending());
        assert_eq!(kw11.tick_count(), 1);
    }

    #[test]
    fn test_kw11_clear_interrupt() {
        let mut kw11 = Kw11::new();
        kw11.write_word(LKS, IE);
        kw11.tick();

        assert!(kw11.interrupt_pending());

        kw11.clear_interrupt();

        assert!(!kw11.interrupt_pending());
    }

    #[test]
    fn test_kw11_disable_clears_pending() {
        let mut kw11 = Kw11::new();
        kw11.write_word(LKS, IE);
        kw11.tick();

        assert!(kw11.interrupt_pending());

        // Disable interrupts
        kw11.write_word(LKS, Word::zero());

        assert!(!kw11.interrupt_pending());
    }

    #[test]
    fn test_kw11_read_lks() {
        let mut kw11 = Kw11::new();
        kw11.write_word(LKS, IE);

        let value = kw11.read_word(LKS);

        assert_eq!(value.as_u16(), 0o000100);
    }
}
