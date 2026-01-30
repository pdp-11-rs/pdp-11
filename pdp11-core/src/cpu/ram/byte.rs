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
    fn test_add_basic() {
        let a = Byte::from(5u8);
        let b = Byte::from(10u8);
        let result = a + b;
        assert_eq!(result.as_u8(), 15);
    }

    #[test]
    fn test_add_wrapping() {
        let a = Byte::from(0xFFu8);
        let b = Byte::from(1u8);
        let result = a + b;
        assert_eq!(result.as_u8(), 0);
    }

    #[test]
    fn test_add_max_values() {
        let a = Byte::from(0xFFu8);
        let b = Byte::from(0xFFu8);
        let result = a + b;
        assert_eq!(result.as_u8(), 0xFEu8);
    }

    #[test]
    fn test_sub_basic() {
        let a = Byte::from(10u8);
        let b = Byte::from(5u8);
        let result = a - b;
        assert_eq!(result.as_u8(), 5);
    }

    #[test]
    fn test_sub_wrapping() {
        let a = Byte::from(0u8);
        let b = Byte::from(1u8);
        let result = a - b;
        assert_eq!(result.as_u8(), 0xFFu8);
    }

    #[test]
    fn test_sub_same_values() {
        let a = Byte::from(42u8);
        let b = Byte::from(42u8);
        let result = a - b;
        assert_eq!(result.as_u8(), 0);
    }

    #[test]
    fn test_add_assign_basic() {
        let mut a = Byte::from(5u8);
        a += Byte::from(10u8);
        assert_eq!(a.as_u8(), 15);
    }

    #[test]
    fn test_add_assign_wrapping() {
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
    fn test_sub_assign_basic() {
        let mut a = Byte::from(10u8);
        a -= Byte::from(5u8);
        assert_eq!(a.as_u8(), 5);
    }

    #[test]
    fn test_sub_assign_wrapping() {
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
    fn test_add_vs_add_assign_consistency() {
        let a = Byte::from(123u8);
        let b = Byte::from(45u8);

        let result1 = a + b;

        let mut result2 = a;
        result2 += b;

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_sub_vs_sub_assign_consistency() {
        let a = Byte::from(200u8);
        let b = Byte::from(50u8);

        let result1 = a - b;

        let mut result2 = a;
        result2 -= b;

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_add_zero_identity() {
        let a = Byte::from(42u8);
        let zero = Byte::from(0u8);
        assert_eq!(a + zero, a);
    }

    #[test]
    fn test_sub_zero_identity() {
        let a = Byte::from(42u8);
        let zero = Byte::from(0u8);
        assert_eq!(a - zero, a);
    }

    #[test]
    fn test_add_commutative() {
        let a = Byte::from(123u8);
        let b = Byte::from(45u8);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn test_byte_wrapping_chain() {
        let mut a = Byte::from(0xF0u8);
        a += 16u8; // 0x00
        a += 10u8; // 0x0A
        a -= 20u8; // 0xF6 (wraps)
        assert_eq!(a.as_u8(), 0xF6u8);
    }
}
