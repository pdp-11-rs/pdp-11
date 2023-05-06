use super::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Word {
    pub(super) le: [u8; 2],
}

impl Word {
    #[inline]
    pub fn as_u16(&self) -> u16 {
        u16::from_le_bytes(self.le)
    }
}
// impl ops::Deref for Word {
//     type Target = u16;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

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
        Self::from_le_bytes(word.le)
    }
}

impl From<Byte> for Word {
    #[inline]
    fn from(byte: Byte) -> Self {
        byte.as_u16().into()
    }
}

impl From<Word> for usize {
    #[inline]
    fn from(word: Word) -> Self {
        u16::from_le_bytes(word.le).into()
    }
}

impl ops::AddAssign for Word {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        // self.le = (u16::from_le_bytes(self.le) + u16::from_le_bytes(rhs.le)).to_le_bytes();
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
}
