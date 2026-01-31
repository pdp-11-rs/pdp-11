use crate::Byte;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Word {
    le: [Byte; 2],
}

impl Word {
    /// Constant for zero
    pub const ZERO: Self = Self::zero();

    /// Constant for one
    pub const ONE: Self = Self::from_u16(1);

    /// Constant for two (commonly used for word-size increments)
    pub const TWO: Self = Self::from_u16(2);

    /// Maximum positive value in PDP-11 two's complement (0o077777 = 32767)
    pub const MAX_POSITIVE: Self = Self::from_u16(0o077777);

    /// Minimum negative value in PDP-11 two's complement (0o100000 = -32768)
    pub const MIN_NEGATIVE: Self = Self::from_u16(0o100000);

    /// Maximum unsigned value (0o177777 = 65535)
    pub const MAX_UNSIGNED: Self = Self::from_u16(0o177777);

    #[inline]
    pub const fn zero() -> Self {
        Self {
            le: [Byte::zero(), Byte::zero()],
        }
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        u16::from_le_bytes([self.le[0].as_u8(), self.le[1].as_u8()])
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.as_u16() as usize
    }

    #[inline]
    pub fn as_i16(&self) -> i16 {
        i16::from_le_bytes([self.le[0].as_u8(), self.le[1].as_u8()])
    }

    /// Convert to i32, sign-extending from 16-bit
    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.as_i16() as i32
    }

    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        let [lo, hi] = value.to_le_bytes();
        let le = [Byte::from_u8(lo), Byte::from_u8(hi)];
        Self { le }
    }

    #[inline]
    pub fn from_i16(value: i16) -> Self {
        let [lo, hi] = value.to_le_bytes();
        let le = [Byte::from_u8(lo), Byte::from_u8(hi)];
        Self { le }
    }

    /// Create Word from lower 16 bits of i32
    #[inline]
    pub fn from_i32(value: i32) -> Self {
        Self::from_i16(value as i16)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.le[0].is_zero() && self.le[1].is_zero()
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.le[1].is_negative()
    }

    pub fn lo_byte(&self) -> Byte {
        self.le[0]
    }

    pub fn hi_byte(&self) -> Byte {
        self.le[1]
    }

    pub fn lo_byte_mut(&mut self) -> &mut Byte {
        &mut self.le[0]
    }

    pub fn hi_byte_mut(&mut self) -> &mut Byte {
        &mut self.le[1]
    }

    /// Get a reference to a byte by index (0 = low byte, 1 = high byte)
    pub fn byte_ref(&self, index: usize) -> &Byte {
        &self.le[index]
    }

    /// Get a mutable reference to a byte by index (0 = low byte, 1 = high byte)
    pub fn byte_ref_mut(&mut self, index: usize) -> &mut Byte {
        &mut self.le[index]
    }

    pub fn swab(&mut self) {
        self.le.swap(0, 1);
    }
}

impl From<u16> for Word {
    #[inline]
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

impl From<Word> for u16 {
    #[inline]
    fn from(word: Word) -> Self {
        word.as_u16()
    }
}

impl From<i16> for Word {
    #[inline]
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<i32> for Word {
    #[inline]
    fn from(value: i32) -> Self {
        Self::from_u16(value as u16)
    }
}

impl From<Word> for i16 {
    #[inline]
    fn from(word: Word) -> Self {
        word.as_i16()
    }
}

impl From<Byte> for Word {
    #[inline]
    fn from(byte: Byte) -> Self {
        byte.sign_extend().into()
    }
}

impl From<Word> for Byte {
    #[inline]
    fn from(word: Word) -> Self {
        word.le[0]
    }
}

impl From<Word> for usize {
    #[inline]
    fn from(word: Word) -> Self {
        word.as_usize()
    }
}

impl PartialOrd for Word {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_u16().cmp(&other.as_u16())
    }
}

impl ops::Add for Word {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.as_u16().wrapping_add(rhs.as_u16()).into()
    }
}

impl ops::Sub for Word {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.as_u16().wrapping_sub(rhs.as_u16()).into()
    }
}

impl ops::BitAnd for Word {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] & rhs.le[0], self.le[1] & rhs.le[1]];
        Self { le }
    }
}

impl ops::BitOr for Word {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] | rhs.le[0], self.le[1] | rhs.le[1]];
        Self { le }
    }
}

impl ops::BitXor for Word {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] ^ rhs.le[0], self.le[1] ^ rhs.le[1]];
        Self { le }
    }
}

impl ops::Not for Word {
    type Output = Self;

    fn not(self) -> Self::Output {
        let le = [!self.le[0], !self.le[1]];
        Self { le }
    }
}

impl ops::Neg for Word {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_u16(0u16.wrapping_sub(self.as_u16()))
    }
}

impl ops::Shl<u32> for Word {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self::from_u16(self.as_u16() << rhs)
    }
}

impl ops::Shr<u32> for Word {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self::from_u16(self.as_u16() >> rhs)
    }
}

impl ops::AddAssign for Word {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<u16> for Word {
    fn add_assign(&mut self, rhs: u16) {
        *self = *self + Self::from_u16(rhs);
    }
}

impl ops::SubAssign for Word {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign<u16> for Word {
    fn sub_assign(&mut self, rhs: u16) {
        *self = *self - Self::from_u16(rhs);
    }
}

impl ops::BitAndAssign for Word {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl ops::BitOrAssign for Word {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl ops::BitXorAssign for Word {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_u16().fmt(f)
    }
}

impl fmt::Octal for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_u16().fmt(f)
    }
}
