use std::marker::PhantomData;
use std::ops::Range;

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Word {
    le: [Byte; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address<M>(Word, PhantomData<M>);

impl<M> Address<M>
where
    M: MemoryAcceess,
{
    pub fn range(&self) -> Range<usize> {
        self.0.address_range::<M>()
    }

    pub fn word_index(&self) -> usize {
        self.0.as_usize() / 2
    }

    pub fn byte_index(&self) -> (usize, usize) {
        let addr = self.0.as_usize();
        (addr / 2, addr % 2)
    }

    pub const fn from_u16(address: u16) -> Self {
        Self(Word::from_u16(address), PhantomData)
    }

    /// Get the u16 value of this address
    pub fn as_u16(&self) -> u16 {
        self.0.as_u16()
    }
}

impl Word {
    #[inline]
    pub const fn zero() -> Self {
        Self {
            le: [Byte::zero(), Byte::zero()],
        }
    }

    pub fn byte(&self, byte: usize) -> &Byte {
        match byte {
            0 => self.lo(),
            1 => self.hi(),
            other => panic!("byte: invalid byte index ({other}) in word"),
        }
    }

    pub fn byte_mut(&mut self, byte: usize) -> &mut Byte {
        match byte {
            0 => self.lo_mut(),
            1 => self.hi_mut(),
            other => panic!("byte_mut: invalid byte index ({other}) in word"),
        }
    }

    pub fn clear(&mut self) {
        self.le[0].clear();
        self.le[1].clear();
    }

    fn lo(&self) -> &Byte {
        &self.le[0]
    }

    fn hi(&self) -> &Byte {
        &self.le[1]
    }

    fn lo_mut(&mut self) -> &mut Byte {
        &mut self.le[0]
    }

    fn hi_mut(&mut self) -> &mut Byte {
        &mut self.le[1]
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
    pub fn swab(&mut self) {
        self.le.swap(0, 1);
    }

    #[inline]
    pub fn address_range<M>(&self) -> Range<usize>
    where
        M: MemoryAcceess,
    {
        let address = self.as_usize();
        address..address + M::SIZE
    }

    #[inline]
    pub fn address<M>(self) -> Address<M> {
        Address(self, PhantomData)
    }

    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        let [lo, hi] = value.to_le_bytes();
        let le = [Byte::from_u8(lo), Byte::from_u8(hi)];
        Self { le }
    }
}

impl From<u16> for Word {
    #[inline]
    fn from(value: u16) -> Self {
        let bytes = value.to_le_bytes();
        Self::from_le_bytes(&bytes)
    }
}

impl From<Word> for u16 {
    #[inline]
    fn from(word: Word) -> Self {
        word.as_u16()
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

impl ops::AddAssign for Word {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let [lo, hi] = self.as_u16().wrapping_add(rhs.as_u16()).to_le_bytes();
        self.le = [Byte::from(lo), Byte::from(hi)];
    }
}

impl ops::AddAssign<u16> for Word {
    #[inline]
    fn add_assign(&mut self, rhs: u16) {
        let [lo, hi] = self.as_u16().wrapping_add(rhs).to_le_bytes();
        self.le = [Byte::from(lo), Byte::from(hi)];
    }
}

impl ops::AddAssign<usize> for Word {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        let [lo, hi] = self.as_u16().wrapping_add(rhs as u16).to_le_bytes();
        self.le = [Byte::from(lo), Byte::from(hi)];
    }
}

impl ops::AddAssign<u8> for Word {
    #[inline]
    fn add_assign(&mut self, rhs: u8) {
        *self += rhs as u16;
    }
}

impl ops::SubAssign for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let [lo, hi] = self.as_u16().wrapping_sub(rhs.as_u16()).to_le_bytes();
        self.le = [Byte::from(lo), Byte::from(hi)];
    }
}

impl ops::SubAssign<u16> for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: u16) {
        let [lo, hi] = self.as_u16().wrapping_sub(rhs).to_le_bytes();
        self.le = [Byte::from(lo), Byte::from(hi)];
    }
}

impl ops::SubAssign<u8> for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: u8) {
        *self -= rhs as u16;
    }
}

impl ops::SubAssign<usize> for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        let [lo, hi] = self.as_u16().wrapping_sub(rhs as u16).to_le_bytes();
        self.le = [Byte::from(lo), Byte::from(hi)];
    }
}

impl MemoryAcceess for Word {
    type LittleEndian = [Byte; Self::SIZE];
    const SIZE: usize = 2;

    fn from_le_bytes(bytes: &[u8]) -> Self {
        let le = [Byte::from(bytes[0]), Byte::from(bytes[1])];
        Self { le }
    }

    fn to_le(&self) -> Self::LittleEndian {
        self.le
    }

    fn as_le_bytes(&self) -> &[u8] {
        todo!("<Word as MemoryAccess>::as_le_bytes()");
    }

    fn is_zero(&self) -> bool {
        self.le[0].is_zero() && self.le[1].is_zero()
    }

    fn is_negative(&self) -> bool {
        self.le[1].is_negative()
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

impl<M> fmt::Display for Address<M>
where
    M: MemoryAcceess,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0.as_u16();
        let size = match M::SIZE {
            1 => "BYTE",
            2 => "WORD",
            other => panic!("Unsupported M::SIZE {other}"),
        };
        format!("{size} @ {value:#08o}").fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic() {
        let a = Word::from(5u16);
        let b = Word::from(10u16);
        let result = a + b;
        assert_eq!(result.as_u16(), 15);
    }

    #[test]
    fn add_wrapping() {
        // Test that addition wraps around at 16-bit boundary
        let a = Word::from(0xFFFFu16);
        let b = Word::from(1u16);
        let result = a + b;
        assert_eq!(result.as_u16(), 0);
    }

    #[test]
    fn add_max_values() {
        let a = Word::from(0xFFFFu16);
        let b = Word::from(0xFFFFu16);
        let result = a + b;
        assert_eq!(result.as_u16(), 0xFFFEu16);
    }

    #[test]
    fn sub_basic() {
        let a = Word::from(10u16);
        let b = Word::from(5u16);
        let result = a - b;
        assert_eq!(result.as_u16(), 5);
    }

    #[test]
    fn sub_wrapping() {
        // Test that subtraction wraps around at 16-bit boundary
        let a = Word::from(0u16);
        let b = Word::from(1u16);
        let result = a - b;
        assert_eq!(result.as_u16(), 0xFFFFu16);
    }

    #[test]
    fn sub_same_values() {
        let a = Word::from(42u16);
        let b = Word::from(42u16);
        let result = a - b;
        assert_eq!(result.as_u16(), 0);
    }

    #[test]
    fn add_assign_basic() {
        let mut a = Word::from(5u16);
        a += Word::from(10u16);
        assert_eq!(a.as_u16(), 15);
    }

    #[test]
    fn add_assign_wrapping() {
        let mut a = Word::from(0xFFFFu16);
        a += Word::from(1u16);
        assert_eq!(a.as_u16(), 0);
    }

    #[test]
    fn test_add_assign_u16() {
        let mut a = Word::from(100u16);
        a += 50u16;
        assert_eq!(a.as_u16(), 150);
    }

    #[test]
    fn test_add_assign_u16_wrapping() {
        let mut a = Word::from(0xFFFEu16);
        a += 5u16;
        assert_eq!(a.as_u16(), 3);
    }

    #[test]
    fn add_assign_usize() {
        let mut a = Word::from(100u16);
        a += 50usize;
        assert_eq!(a.as_u16(), 150);
    }

    #[test]
    fn test_add_assign_u8() {
        let mut a = Word::from(100u16);
        a += 50u8;
        assert_eq!(a.as_u16(), 150);
    }

    #[test]
    fn sub_assign_basic() {
        let mut a = Word::from(10u16);
        a -= Word::from(5u16);
        assert_eq!(a.as_u16(), 5);
    }

    #[test]
    fn sub_assign_wrapping() {
        let mut a = Word::from(0u16);
        a -= Word::from(1u16);
        assert_eq!(a.as_u16(), 0xFFFFu16);
    }

    #[test]
    fn test_sub_assign_u16() {
        let mut a = Word::from(100u16);
        a -= 50u16;
        assert_eq!(a.as_u16(), 50);
    }

    #[test]
    fn test_sub_assign_u16_wrapping() {
        let mut a = Word::from(3u16);
        a -= 5u16;
        assert_eq!(a.as_u16(), 0xFFFEu16);
    }

    #[test]
    fn sub_assign_usize() {
        let mut a = Word::from(100u16);
        a -= 50usize;
        assert_eq!(a.as_u16(), 50);
    }

    #[test]
    fn test_sub_assign_u8() {
        let mut a = Word::from(100u16);
        a -= 50u8;
        assert_eq!(a.as_u16(), 50);
    }

    #[test]
    fn add_vs_add_assign_consistency() {
        // Verify that Add and AddAssign produce the same results
        let a = Word::from(123u16);
        let b = Word::from(456u16);

        let result1 = a + b;

        let mut result2 = a;
        result2 += b;

        assert_eq!(result1, result2);
    }

    #[test]
    fn sub_vs_sub_assign_consistency() {
        // Verify that Sub and SubAssign produce the same results
        let a = Word::from(456u16);
        let b = Word::from(123u16);

        let result1 = a - b;

        let mut result2 = a;
        result2 -= b;

        assert_eq!(result1, result2);
    }

    #[test]
    fn add_zero_identity() {
        let a = Word::from(42u16);
        let zero = Word::from(0u16);
        assert_eq!(a + zero, a);
    }

    #[test]
    fn sub_zero_identity() {
        let a = Word::from(42u16);
        let zero = Word::from(0u16);
        assert_eq!(a - zero, a);
    }

    #[test]
    fn add_commutative() {
        let a = Word::from(123u16);
        let b = Word::from(456u16);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn address_arithmetic_wrapping() {
        // Simulate PDP-11 address wrapping scenarios

        // Address at end of memory + offset
        let base = Word::from(0xFFF0u16);
        let offset = Word::from(0x20u16);
        let result = base + offset;
        assert_eq!(result.as_u16(), 0x0010u16); // Wraps to low memory

        // Address at start of memory - offset
        let base = Word::from(0x0010u16);
        let offset = Word::from(0x20u16);
        let result = base - offset;
        assert_eq!(result.as_u16(), 0xFFF0u16); // Wraps to high memory
    }

    #[test]
    fn pc_increment_wrapping() {
        // Simulate PC (Program Counter) wrapping
        let mut pc = Word::from(0xFFFEu16);
        pc += 2u16; // Fetch next instruction
        assert_eq!(pc.as_u16(), 0); // PC wraps to 0
    }

    #[test]
    fn multiple_operations() {
        let mut a = Word::from(100u16);
        a += 50u16;
        a -= 30u16;
        a += 10u16;
        assert_eq!(a.as_u16(), 130);
    }

    #[test]
    fn wrapping_chain() {
        let mut a = Word::from(0xFFF0u16);
        a += 16u16; // 0x0000
        a += 10u16; // 0x000A
        a -= 20u16; // 0xFFF6 (wraps)
        assert_eq!(a.as_u16(), 0xFFF6u16);
    }
}
