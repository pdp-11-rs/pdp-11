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
    pub fn get<M>(&mut self, idx: Register, mode: RegisterAddressingMode) -> Word
    where
        M: MemoryAcceess,
    {
        use RegisterAddressingMode::*;

        let idx = idx as usize;
        let output = self.0[idx];
        match mode {
            Register => {}
            RegisterDeferred => {}
            Autoincrement => self.0[idx] += M::SIZE,
            AutoincrementDeferred => self.0[idx] += M::SIZE,
            Autodecrement => self.0[idx] -= M::SIZE,
            AutodecrementDeferred => self.0[idx] -= M::SIZE,
            Index => {}
            IndexDeferred => {}
        }
        output
    }

    pub fn set<M>(&mut self, idx: Register, mode: RegisterAddressingMode, data: M)
    where
        M: MemoryAcceess,
    {
        use RegisterAddressingMode::*;

        let idx = idx as usize;
        let size = M::SIZE as u16;
        let word = data.into();
        match mode {
            Register => self.0[idx] = word,
            RegisterDeferred => {}
            Autoincrement => self.0[idx] += size,
            AutoincrementDeferred => self.0[idx] += size,
            Autodecrement => self.0[idx] -= size,
            AutodecrementDeferred => self.0[idx] -= size,
            Index => {}
            IndexDeferred => {}
        }
    }

    pub fn inc<M>(&mut self, register: Register)
    where
        M: MemoryAcceess,
    {
        self[register] += M::SIZE;
    }

    pub fn dec<M>(&mut self, register: Register)
    where
        M: MemoryAcceess,
    {
        self[register] -= M::SIZE;
    }
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
