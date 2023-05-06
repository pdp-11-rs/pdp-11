use super::*;

pub use byte::Byte;
pub use word::Word;

mod byte;
mod word;

#[derive(Debug)]
pub struct Ram([u8; 64 * 1024]);

impl Ram {
    pub fn new() -> Self {
        Self([0; 64 * 1024])
    }

    // #[inline]
    // pub fn load_byte(&self, address: Word) -> Byte {
    //     Byte(self.0[address.0 as usize])
    // }

    // #[inline]
    // pub fn load_word(&self, address: Word) -> Word {
    //     let lo = address.0 as usize;
    //     let hi = lo + 1;
    //     Word(u16::from_le_bytes([self.0[lo], self.0[hi]]))
    // }

    #[inline]
    pub fn load_range<M>(&self, address: Word) -> &[u8]
    where
        M: MemoryAcceess,
    {
        let addr = address.into();
        &self.0[addr..(addr + M::SIZE)]
    }

    #[inline]
    pub fn store_range<M>(&mut self, address: Word, data: M)
    where
        M: MemoryAcceess,
    {
        let addr = address.into();
        let data = data.as_le_bytes();
        self.0[addr..(addr + M::SIZE)].copy_from_slice(data);
    }

    #[inline]
    pub fn load<M>(&self, address: Word) -> M
    where
        M: MemoryAcceess,
    {
        let addr = address.into();
        println!("Loading {} bytes from {addr}", M::SIZE);
        let bytes = &self.0[addr..(addr + M::SIZE)];
        M::from_le_bytes(bytes)
    }

    // #[inline]
    // pub fn store<M>(&mut self, address: Word, data: M)
    // where
    //     M: MemoryAcceess,
    // {
    //     let addr = address.0 as usize;
    //     println!("Storing {} bytes to {addr}", M::SIZE);
    //     let bytes = data.to_le_bytes();
    //     &mut self.0[addr..(addr + M::SIZE)].copy_from_slice(&bytes);
    // }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
