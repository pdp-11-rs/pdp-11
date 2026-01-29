use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Byte {
    pub(super) le: [u8; 1],
}

impl Byte {
    pub const fn zero() -> Self {
        Byte { le: [0] }
    }

    #[inline]
    pub const fn as_u8(&self) -> u8 {
        self.le[0]
    }

    #[inline]
    pub const fn sign_extend(&self) -> u16 {
        ((self.le[0] as i8) as i16) as u16
    }

    #[inline]
    pub const fn as_u16(&self) -> u16 {
        self.le[0] as u16
    }

    #[inline]
    pub fn clear(&mut self) {
        self.le[0] = 0;
    }

    #[inline]
    pub(super) const fn from_u8(value: u8) -> Self {
        Self {
            le: value.to_le_bytes(),
        }
    }
}

impl From<u8> for Byte {
    #[inline]
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<Byte> for u8 {
    #[inline]
    fn from(byte: Byte) -> Self {
        Self::from_le_bytes(byte.le)
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

impl ops::BitAnd for Byte {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] & rhs.le[0]];
        Self { le }
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

    fn is_zero(&self) -> bool {
        self.le[0] == 0
    }

    fn is_negative(&self) -> bool {
        (self.le[0] as i8).is_negative()
    }
}
