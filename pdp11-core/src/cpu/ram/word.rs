use std::ops::Range;

use super::*;

// Emulator-specific extensions to Word
pub trait WordExt {
    fn byte(&self, byte: usize) -> &Byte;
    fn byte_mut(&mut self, byte: usize) -> &mut Byte;
    fn clear(&mut self);
    fn address<M>(&self) -> Address<M>;
    fn address_range<M: MemoryAcceess>(&self) -> Range<usize>;
}

impl WordExt for Word {
    fn byte(&self, byte: usize) -> &Byte {
        match byte {
            0 | 1 => self.byte_ref(byte),
            other => panic!("byte: invalid byte index ({other}) in word"),
        }
    }

    fn byte_mut(&mut self, byte: usize) -> &mut Byte {
        match byte {
            0 | 1 => self.byte_ref_mut(byte),
            other => panic!("byte_mut: invalid byte index ({other}) in word"),
        }
    }

    #[allow(clippy::use_self)]
    fn clear(&mut self) {
        *self = Word::ZERO;
    }

    fn address<M>(&self) -> Address<M> {
        Address::from_u16(self.as_u16())
    }

    fn address_range<M: MemoryAcceess>(&self) -> Range<usize> {
        let address = self.as_usize();
        address..address + M::SIZE
    }
}

impl MemoryAcceess for Word {
    const SIZE: usize = 2;
    type LittleEndian = u16;

    fn from_le_bytes(bytes: &[u8]) -> Self {
        Self::from_u16(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn to_le(&self) -> Self::LittleEndian {
        self.as_u16()
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.is_negative()
    }
}
