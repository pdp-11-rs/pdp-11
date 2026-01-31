use super::*;

pub use pdp11_common::Register;

#[derive(Debug)]
pub struct Registers([Word; 8]);

impl Registers {
    pub fn reset(&mut self) {
        *self = Self([Word::zero(); 8])
    }

    pub fn get_inc<M>(&mut self, register: Register) -> Word
    where
        M: MemoryAcceess,
    {
        let word = self[register];
        self[register] += M::SIZE as u16;
        word
    }

    pub fn dec_get<M>(&mut self, register: Register) -> Word
    where
        M: MemoryAcceess,
    {
        self[register] -= M::SIZE as u16;
        self[register]
    }
}

impl ops::Index<Register> for Registers {
    type Output = Word;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[index.to_code() as usize]
    }
}

impl ops::IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[index.to_code() as usize]
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self([Word::zero(); 8])
    }
}
