use super::*;

#[derive(Debug)]
pub enum Instruction {
    Mov(Source, Destination),
}

impl Instruction {
    fn mov(opcode: u16) -> Self {
        let src = Source::from_double_operand(opcode);
        let dst = Destination::from_double_operand(opcode);
        Self::Mov(src, dst)
    }
}

impl From<Word> for Instruction {
    fn from(opcode: Word) -> Self {
        match opcode.as_u16() {
            code @ 0o01_00_00..=0o01_77_77 => Instruction::mov(code),
            other => todo!("Opcode {other:0o} is not supported yet"),
        }
    }
}
