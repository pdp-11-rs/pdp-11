use std::marker::PhantomData;
use std::ops::Range;

use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Word {
    pub(super) le: [u8; 2],
}

#[derive(Debug)]
pub struct Address<M>(Word, PhantomData<M>);

impl<M> Address<M>
where
    M: MemoryAcceess,
{
    pub fn range(&self) -> Range<usize> {
        self.0.address_range::<M>()
    }
}

impl Word {
    #[inline]
    pub fn as_u16(&self) -> u16 {
        u16::from_le_bytes(self.le)
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        u16::from_le_bytes(self.le) as usize
    }

    #[inline]
    pub fn address_range<M>(&self) -> Range<usize>
    where
        M: MemoryAcceess,
    {
        let address = self.as_usize();
        address..address + M::SIZE
    }

    pub fn address<M>(self) -> Address<M> {
        Address(self, PhantomData)
    }
}

impl From<u16> for Word {
    #[inline]
    fn from(value: u16) -> Self {
        let le = value.to_le_bytes();
        Self { le }
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

impl From<Word> for usize {
    #[inline]
    fn from(word: Word) -> Self {
        word.as_usize()
    }
}

impl ops::Sub for Word {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        (self.as_u16() - rhs.as_u16()).into()
    }
}

impl ops::BitAnd for Word {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let le = [self.le[0] & rhs.le[0], self.le[1] & rhs.le[1]];
        Self { le }
    }
}

impl ops::AddAssign for Word {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.le = (self.as_u16() + u16::from(rhs)).to_le_bytes();
    }
}

impl ops::AddAssign<u16> for Word {
    #[inline]
    fn add_assign(&mut self, rhs: u16) {
        self.le = (self.as_u16() + rhs).to_le_bytes();
    }
}

impl ops::AddAssign<usize> for Word {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.le = (self.as_u16() + rhs as u16).to_le_bytes();
    }
}

impl ops::SubAssign for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.le = (self.as_u16() - u16::from(rhs)).to_le_bytes();
    }
}

impl ops::SubAssign<u16> for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: u16) {
        self.le = (self.as_u16() - rhs).to_le_bytes()
    }
}

impl ops::SubAssign<usize> for Word {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.le = (self.as_u16() - rhs as u16).to_le_bytes()
    }
}

impl MemoryAcceess for Word {
    type LittleEndian = [u8; Self::SIZE];
    const SIZE: usize = 2;

    fn from_le_bytes(bytes: &[u8]) -> Self {
        let le = [bytes[0], bytes[1]];
        Self { le }
    }

    fn to_le(&self) -> Self::LittleEndian {
        self.le
    }

    fn as_le_bytes(&self) -> &[u8] {
        &self.le
    }

    fn is_zero(&self) -> bool {
        self.le[0] == 0 && self.le[1] == 0
    }

    fn is_negative(&self) -> bool {
        (self.le[1] as i8).is_negative()
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
