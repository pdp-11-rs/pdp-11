use super::*;

#[derive(Debug)]
pub enum Instruction {
    Mov(Source, Destination),
    Invalid(u16),
}

impl Instruction {
    fn mov(opcode: u16) -> Self {
        let src = Source::from_double_operand(opcode);
        let dst = Destination::from_double_operand(opcode);
        Self::Mov(src, dst)
    }

    fn disassemble(&self) -> String {
        match self {
            Self::Mov(src, dst) => format!("MOV\t{src}, {dst}"),
            Self::Invalid(opcode) => format!("Invalid opcode {opcode:#08o}"),
        }
    }
}

impl From<Word> for Instruction {
    fn from(opcode: Word) -> Self {
        match opcode.as_u16() {
            code @ 0o01_00_00..=0o01_77_77 => Instruction::mov(code),
            other => Instruction::Invalid(other),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.disassemble().fmt(f)
    }
}
