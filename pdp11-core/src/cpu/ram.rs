use super::*;

pub use byte::Byte;
pub use word::Address;
pub use word::Word;

mod byte;
mod word;

#[derive(Debug)]
pub struct Ram([Word; 32 * 1024]);

impl Ram {
    pub fn reset(&mut self) {
        *self = Self([Word::zero(); 32 * 1024])
    }

    pub fn new() -> Self {
        Self([Word::zero(); 32 * 1024])
    }

    #[inline]
    pub fn word(&self, address: Address<Word>) -> &Word {
        println!("Loading {address}");
        &self.0[address.word_index()]
    }

    #[inline]
    pub fn word_mut(&mut self, address: Address<Word>) -> &mut Word {
        println!("Storing {address}");
        &mut self.0[address.word_index()]
    }

    #[inline]
    pub fn byte(&self, address: Address<Byte>) -> &Byte {
        println!("Loading {address}");
        let (index, byte) = address.byte_index();
        self.0[index].byte(byte)
    }

    #[inline]
    pub fn byte_mut(&mut self, address: Address<Byte>) -> &mut Byte {
        println!("Storing {address}");
        let (index, byte) = address.byte_index();
        self.0[index].byte_mut(byte)
    }

    /// Direct write to RAM, bypassing memory-mapped I/O
    /// Used for DMA operations from RK controller
    pub(super) fn write_direct(&mut self, address: Address<Word>, value: Word) {
        self.0[address.word_index()] = value;
    }

    // #[inline]
    // pub fn load<M>(&self, address: Address<M>) -> M
    // where
    //     M: MemoryAcceess,
    // {
    //     println!("Loading {address}");
    //     M::from_le_bytes(&self[address])
    // }

    // #[inline]
    // pub fn store<M>(&mut self, address: Address<M>, data: M)
    // where
    //     M: MemoryAcceess,
    // {
    //     println!("Storing {data:08o} {address}");
    //     self[address].copy_from_slice(data.as_le_bytes());
    // }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Index<Address<Word>> for Ram {
    type Output = Word;

    #[inline]
    fn index(&self, address: Address<Word>) -> &Self::Output {
        self.word(address)
    }
}

impl ops::IndexMut<Address<Word>> for Ram {
    #[inline]
    fn index_mut(&mut self, address: Address<Word>) -> &mut Self::Output {
        self.word_mut(address)
    }
}

impl ops::Index<Address<Byte>> for Ram {
    type Output = Byte;

    #[inline]
    fn index(&self, address: Address<Byte>) -> &Self::Output {
        self.byte(address)
    }
}

impl ops::IndexMut<Address<Byte>> for Ram {
    #[inline]
    fn index_mut(&mut self, address: Address<Byte>) -> &mut Self::Output {
        self.byte_mut(address)
    }
}
