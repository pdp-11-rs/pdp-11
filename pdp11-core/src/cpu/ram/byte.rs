use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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
        Byte::from_u8(0u8.wrapping_sub(self.as_u8()))
    }
}

impl ops::Shl<u32> for Byte {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Byte::from_u8(self.as_u8() << rhs)
    }
}

impl ops::Shr<u32> for Byte {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Byte::from_u8(self.as_u8() >> rhs)
    }
}

impl ops::ShlAssign<u32> for Byte {
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl ops::ShrAssign<u32> for Byte {
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl ops::BitAndAssign for Byte {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl ops::BitOrAssign for Byte {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl ops::BitXorAssign for Byte {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl ops::Add for Byte {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.as_u8().wrapping_add(rhs.as_u8()).into()
    }
}

impl ops::Sub for Byte {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.as_u8().wrapping_sub(rhs.as_u8()).into()
    }
}

impl ops::AddAssign for Byte {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.le[0] = self.as_u8().wrapping_add(rhs.as_u8());
    }
}

impl ops::AddAssign<u8> for Byte {
    #[inline]
    fn add_assign(&mut self, rhs: u8) {
        self.le[0] = self.as_u8().wrapping_add(rhs);
    }
}

impl ops::SubAssign for Byte {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.le[0] = self.as_u8().wrapping_sub(rhs.as_u8());
    }
}

impl ops::SubAssign<u8> for Byte {
    #[inline]
    fn sub_assign(&mut self, rhs: u8) {
        self.le[0] = self.as_u8().wrapping_sub(rhs);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic() {
        let a = Byte::from(5u8);
        let b = Byte::from(10u8);
        let result = a + b;
        assert_eq!(result.as_u8(), 15);
    }

    #[test]
    fn add_wrapping() {
        let a = Byte::from(0xFFu8);
        let b = Byte::from(1u8);
        let result = a + b;
        assert_eq!(result.as_u8(), 0);
    }

    #[test]
    fn add_max_values() {
        let a = Byte::from(0xFFu8);
        let b = Byte::from(0xFFu8);
        let result = a + b;
        assert_eq!(result.as_u8(), 0xFEu8);
    }

    #[test]
    fn sub_basic() {
        let a = Byte::from(10u8);
        let b = Byte::from(5u8);
        let result = a - b;
        assert_eq!(result.as_u8(), 5);
    }

    #[test]
    fn sub_wrapping() {
        let a = Byte::from(0u8);
        let b = Byte::from(1u8);
        let result = a - b;
        assert_eq!(result.as_u8(), 0xFFu8);
    }

    #[test]
    fn sub_same_values() {
        let a = Byte::from(42u8);
        let b = Byte::from(42u8);
        let result = a - b;
        assert_eq!(result.as_u8(), 0);
    }

    #[test]
    fn add_assign_basic() {
        let mut a = Byte::from(5u8);
        a += Byte::from(10u8);
        assert_eq!(a.as_u8(), 15);
    }

    #[test]
    fn add_assign_wrapping() {
        let mut a = Byte::from(0xFFu8);
        a += Byte::from(1u8);
        assert_eq!(a.as_u8(), 0);
    }

    #[test]
    fn test_add_assign_u8() {
        let mut a = Byte::from(100u8);
        a += 50u8;
        assert_eq!(a.as_u8(), 150);
    }

    #[test]
    fn test_add_assign_u8_wrapping() {
        let mut a = Byte::from(0xFEu8);
        a += 5u8;
        assert_eq!(a.as_u8(), 3);
    }

    #[test]
    fn sub_assign_basic() {
        let mut a = Byte::from(10u8);
        a -= Byte::from(5u8);
        assert_eq!(a.as_u8(), 5);
    }

    #[test]
    fn sub_assign_wrapping() {
        let mut a = Byte::from(0u8);
        a -= Byte::from(1u8);
        assert_eq!(a.as_u8(), 0xFFu8);
    }

    #[test]
    fn test_sub_assign_u8() {
        let mut a = Byte::from(100u8);
        a -= 50u8;
        assert_eq!(a.as_u8(), 50);
    }

    #[test]
    fn test_sub_assign_u8_wrapping() {
        let mut a = Byte::from(3u8);
        a -= 5u8;
        assert_eq!(a.as_u8(), 0xFEu8);
    }

    #[test]
    fn add_vs_add_assign_consistency() {
        let a = Byte::from(123u8);
        let b = Byte::from(45u8);

        let result1 = a + b;

        let mut result2 = a;
        result2 += b;

        assert_eq!(result1, result2);
    }

    #[test]
    fn sub_vs_sub_assign_consistency() {
        let a = Byte::from(200u8);
        let b = Byte::from(50u8);

        let result1 = a - b;

        let mut result2 = a;
        result2 -= b;

        assert_eq!(result1, result2);
    }

    #[test]
    fn add_zero_identity() {
        let a = Byte::from(42u8);
        let zero = Byte::from(0u8);
        assert_eq!(a + zero, a);
    }

    #[test]
    fn sub_zero_identity() {
        let a = Byte::from(42u8);
        let zero = Byte::from(0u8);
        assert_eq!(a - zero, a);
    }

    #[test]
    fn add_commutative() {
        let a = Byte::from(123u8);
        let b = Byte::from(45u8);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn byte_wrapping_chain() {
        let mut a = Byte::from(0xF0u8);
        a += 16u8; // 0x00
        a += 10u8; // 0x0A
        a -= 20u8; // 0xF6 (wraps)
        assert_eq!(a.as_u8(), 0xF6u8);
    }

    #[test]
    fn test_neg() {
        assert_eq!((-Byte::from(1u8)).as_u8(), 0xFFu8);
        assert_eq!((-Byte::from(0xFFu8)).as_u8(), 1u8);
        assert_eq!((-Byte::from(0x80u8)).as_u8(), 0x80u8); // -128 in 8-bit
        assert_eq!((-Byte::from(100u8)).as_u8(), 0x9Cu8);
    }

    #[test]
    fn test_shl() {
        assert_eq!((Byte::from(0b1010u8) << 1).as_u8(), 0b10100);
        assert_eq!((Byte::from(0b1010u8) << 4).as_u8(), 0b10100000);
        assert_eq!((Byte::from(0x80u8) << 1).as_u8(), 0); // Overflow
    }

    #[test]
    fn test_shr() {
        assert_eq!((Byte::from(0b10100u8) >> 1).as_u8(), 0b1010);
        assert_eq!((Byte::from(0b10100000u8) >> 4).as_u8(), 0b1010);
        assert_eq!((Byte::from(1u8) >> 1).as_u8(), 0);
    }

    #[test]
    fn test_shl_assign() {
        let mut b = Byte::from(0b1010u8);
        b <<= 1;
        assert_eq!(b.as_u8(), 0b10100);
        b <<= 3;
        assert_eq!(b.as_u8(), 0b10100000);
    }

    #[test]
    fn test_shr_assign() {
        let mut b = Byte::from(0b10100000u8);
        b >>= 1;
        assert_eq!(b.as_u8(), 0b1010000);
        b >>= 3;
        assert_eq!(b.as_u8(), 0b1010);
    }

    #[test]
    fn test_bitand_assign() {
        let mut b = Byte::from(0b1111u8);
        b &= Byte::from(0b1010u8);
        assert_eq!(b.as_u8(), 0b1010);
    }

    #[test]
    fn test_bitor_assign() {
        let mut b = Byte::from(0b1010u8);
        b |= Byte::from(0b0101u8);
        assert_eq!(b.as_u8(), 0b1111);
    }

    #[test]
    fn test_bitxor_assign() {
        let mut b = Byte::from(0b1111u8);
        b ^= Byte::from(0b1010u8);
        assert_eq!(b.as_u8(), 0b0101);
    }
}
