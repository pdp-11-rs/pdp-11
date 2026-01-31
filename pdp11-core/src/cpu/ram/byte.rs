use super::*;

impl MemoryAcceess for Byte {
    const SIZE: usize = 1;
    type LittleEndian = u8;

    fn from_le_bytes(bytes: &[u8]) -> Self {
        Self::from_u8(bytes[0])
    }

    fn to_le(&self) -> Self::LittleEndian {
        self.as_u8()
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.is_negative()
    }
}
