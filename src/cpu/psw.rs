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
