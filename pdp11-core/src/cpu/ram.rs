use super::*;

pub use byte::Byte;
pub use word::Address;
pub use word::Word;

mod byte;
mod word;

const RK11_400: Address<Word> = Address::from_u16(0o177400);
const RK11_402: Address<Word> = Address::from_u16(0o177402);
const RK11_404: Address<Word> = Address::from_u16(0o177404);
const RK11_406: Address<Word> = Address::from_u16(0o177406);
const RK11_410: Address<Word> = Address::from_u16(0o177410);
const RK11_412: Address<Word> = Address::from_u16(0o177412);

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
        match address {
            RK11_400 | RK11_402 | RK11_404 | RK11_406 | RK11_410 | RK11_412 => self.rk(address),
            _ => &self.0[address.word_index()],
        }
    }

    #[inline]
    pub fn word_mut(&mut self, address: Address<Word>) -> &mut Word {
        println!("Storing {address}");
        match address {
            RK11_400 | RK11_402 | RK11_404 | RK11_406 | RK11_410 | RK11_412 => self.rk_mut(address),
            _ => &mut self.0[address.word_index()],
        }
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

    fn rk(&self, address: Address<Word>) -> &Word {
        todo!("RK READ {address}");
    }

    fn rk_mut(&self, address: Address<Word>) -> &mut Word {
        todo!("RK WRITE {address}");
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
