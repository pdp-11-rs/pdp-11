use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Byte {
    le: [u8; 1],
}

impl Byte {
    /// Constant for zero
    pub const ZERO: Self = Self::zero();

    /// Constant for one
    pub const ONE: Self = Self::from_u8(1);

    /// Constant for two
    pub const TWO: Self = Self::from_u8(2);

    /// Maximum positive value in PDP-11 two's complement (0o177 = 127)
    pub const MAX_POSITIVE: Self = Self::from_u8(0o177);

    /// Minimum negative value in PDP-11 two's complement (0o200 = -128)
    pub const MIN_NEGATIVE: Self = Self::from_u8(0o200);

    /// Maximum unsigned value (0o377 = 255)
    pub const MAX_UNSIGNED: Self = Self::from_u8(0o377);

    #[inline]
    pub const fn zero() -> Self {
        Self { le: [0] }
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
    pub const fn from_u8(value: u8) -> Self {
        Self {
            le: value.to_le_bytes(),
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.le[0] == 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.le[0] & 0o200) != 0
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

impl PartialOrd for Byte {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Byte {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_u8().cmp(&other.as_u8())
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

impl ops::BitOr for Byte {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] | rhs.le[0]];
        Self { le }
    }
}

impl ops::BitXor for Byte {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] ^ rhs.le[0]];
        Self { le }
    }
}

impl ops::Not for Byte {
    type Output = Self;

    fn not(self) -> Self::Output {
        let le = [!self.le[0]];
        Self { le }
    }
}

impl ops::Neg for Byte {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_u8(0u8.wrapping_sub(self.as_u8()))
    }
}

impl ops::Add for Byte {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.as_u8().wrapping_add(rhs.as_u8()))
    }
}

impl ops::Sub for Byte {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.as_u8().wrapping_sub(rhs.as_u8()))
    }
}

impl ops::Shl<u8> for Byte {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self::from_u8(self.as_u8() << rhs)
    }
}

impl ops::Shr<u8> for Byte {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self::from_u8(self.as_u8() >> rhs)
    }
}

// Additional shift implementations for integer literals
impl ops::Shl<i32> for Byte {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self::from_u8(self.as_u8() << rhs)
    }
}

impl ops::Shr<i32> for Byte {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self::from_u8(self.as_u8() >> rhs)
    }
}

impl ops::Shl<usize> for Byte {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: usize) -> Self::Output {
        Self::from_u8(self.as_u8() << rhs)
    }
}

impl ops::Shr<usize> for Byte {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: usize) -> Self::Output {
        Self::from_u8(self.as_u8() >> rhs)
    }
}
