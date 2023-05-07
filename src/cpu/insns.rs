use std::borrow::Cow;

use super::*;

#[derive(Debug)]
pub enum Instruction {
    Halt,
    Wait,
    Reset,
    Mov(Source, Destination),
    Invalid(u16),
}

impl Instruction {
    fn mov(opcode: u16) -> Self {
        let src = Source::from_double_operand(opcode);
        let dst = Destination::from_double_operand(opcode);
        Self::Mov(src, dst)
    }

    fn disassemble(&self) -> Cow<'static, str> {
        use Instruction::*;
        match self {
            Halt => "HALT".into(),
            Wait => "WAIT".into(),
            Reset => "RESET".into(),
            Mov(src, dst) => format!("MOV\t{src}, {dst}").into(),
            Invalid(opcode) => format!("Invalid opcode {opcode:#08o}").into(),
        }
    }
}

impl From<Word> for Instruction {
    fn from(opcode: Word) -> Self {
        use Instruction::*;
        match opcode.as_u16() {
            0o000000 => Halt,
            0o000001 => Wait,
            0o000005 => Reset,
            code @ 0o010000..=0o017777 => Self::mov(code),
            other => Instruction::Invalid(other),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.disassemble().fmt(f)
    }
}
