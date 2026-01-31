use super::*;

#[derive(Debug, Default)]
pub struct ProcessorStatusWord {
    carry: bool,
    overflow: bool,
    zero: bool,
    negative: bool,
    trap: bool,
    ipl: u8,
}

impl ProcessorStatusWord {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn clear_flags(&mut self) {
        self.carry = false;
        self.overflow = false;
        self.zero = false;
        self.negative = false;
    }

    /// Get current interrupt priority level (0-7)
    pub fn priority(&self) -> u8 {
        self.ipl & 0x7
    }

    /// Set interrupt priority level (0-7)
    pub fn set_priority(&mut self, level: u8) {
        self.ipl = level & 0x7;
    }

    /// Convert PSW to Word format for stack save/interrupt vectors
    /// Format: bits 15-8: unused, bits 7-5: IPL, bit 4: trap, bits 3-0: NZVC
    pub fn as_word(&self) -> Word {
        let mut value = 0u16;
        if self.carry {
            value |= 0b0001;
        }
        if self.overflow {
            value |= 0b0010;
        }
        if self.zero {
            value |= 0b0100;
        }
        if self.negative {
            value |= 0b1000;
        }
        if self.trap {
            value |= 0b0001_0000;
        }
        value |= (u16::from(self.ipl) & 0x7) << 5;
        Word::from(value)
    }

    /// Load PSW from Word format (from stack restore/interrupt vectors)
    pub fn from_word(&mut self, word: Word) {
        let value = word.as_u16();
        self.carry = (value & 0b0001) != 0;
        self.overflow = (value & 0b0010) != 0;
        self.zero = (value & 0b0100) != 0;
        self.negative = (value & 0b1000) != 0;
        self.trap = (value & 0b0001_0000) != 0;
        self.ipl = ((value >> 5) & 0x7) as u8;
    }
}

impl ops::Index<Flags> for ProcessorStatusWord {
    type Output = bool;

    fn index(&self, index: Flags) -> &Self::Output {
        match index {
            Flags::C => &self.carry,
            Flags::V => &self.overflow,
            Flags::Z => &self.zero,
            Flags::N => &self.negative,
        }
    }
}

impl ops::IndexMut<Flags> for ProcessorStatusWord {
    fn index_mut(&mut self, index: Flags) -> &mut Self::Output {
        match index {
            Flags::C => &mut self.carry,
            Flags::V => &mut self.overflow,
            Flags::Z => &mut self.zero,
            Flags::N => &mut self.negative,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Flags {
    C,
    V,
    Z,
    N,
}
