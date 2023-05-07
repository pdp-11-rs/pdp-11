use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Byte {
    pub(super) le: [u8; 1],
}

impl Byte {
    #[inline]
    pub fn as_u8(&self) -> u8 {
        self.le[0]
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        self.le[0].into()
    }
}

impl From<u8> for Byte {
    #[inline]
    fn from(value: u8) -> Self {
        let le = value.to_le_bytes();
        Self { le }
    }
}

impl From<Byte> for u8 {
    #[inline]
    fn from(byte: Byte) -> Self {
        Self::from_le_bytes(byte.le)
    }
}

impl From<Word> for Byte {
    #[inline]
    fn from(word: Word) -> Self {
        word.le[0].into()
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_u8().fmt(f)
    }
}

impl fmt::Octal for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_u8().fmt(f)
    }
}

impl MemoryAcceess for Byte {
    type LittleEndian = [u8; Self::SIZE];
    const SIZE: usize = 1;

    fn from_le_bytes(bytes: &[u8]) -> Self {
        bytes[0].into()
    }

    fn to_le(&self) -> Self::LittleEndian {
        self.le
    }

    fn as_le_bytes(&self) -> &[u8] {
        &self.le
    }
}
