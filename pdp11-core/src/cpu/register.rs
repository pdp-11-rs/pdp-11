use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    SP,
    PC,
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::SP,
            7 => Self::PC,
            other => panic!("Invalid register code {other}"),
        }
    }
}

impl Register {
    pub fn as_str(&self) -> &'static str {
        match self {
            R0 => "R0",
            R1 => "R1",
            R2 => "R2",
            R3 => "R3",
            R4 => "R4",
            R5 => "R5",
            SP => "SP",
            PC => "PC",
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Registers([Word; 8]);

impl Registers {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn get_inc<M>(&mut self, register: Register) -> Word
    where
        M: MemoryAcceess,
    {
        let word = self[register];
        self[register] += M::SIZE;
        word
    }

    pub fn dec_get<M>(&mut self, register: Register) -> Word
    where
        M: MemoryAcceess,
    {
        self[register] += M::SIZE;
        self[register]
    }

    // pub fn inc<M>(&mut self, register: Register)
    // where
    //     M: MemoryAcceess,
    // {
    //     self[register] += M::SIZE;
    // }

    // pub fn dec<M>(&mut self, register: Register)
    // where
    //     M: MemoryAcceess,
    // {
    //     self[register] -= M::SIZE;
    // }
}

impl ops::Index<Register> for Registers {
    type Output = Word;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl ops::IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
