use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
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
            6 => Self::R6,
            7 => Self::R7,
            other => panic!("Invalid register code {other}"),
        }
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
}
