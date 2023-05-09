use super::*;

pub use byte::Byte;
pub use word::Address;
pub use word::Word;

mod byte;
mod word;

#[derive(Debug)]
pub struct Ram([Word; 32 * 1024]);

impl Ram {
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

// impl<M> ops::Index<Address<M>> for Ram
// where
//     M: MemoryAcceess,
// {
//     type Output = [u8];

//     fn index(&self, index: Address<M>) -> &Self::Output {
//         &self.0[index.range()]
//     }
// }

// impl<M> ops::IndexMut<Address<M>> for Ram
// where
//     M: MemoryAcceess,
// {
//     fn index_mut(&mut self, index: Address<M>) -> &mut Self::Output {
//         &mut self.0[index.range()]
//     }
// }
