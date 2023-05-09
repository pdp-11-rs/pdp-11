use super::*;

#[derive(Debug)]
pub enum Instruction {
    Halt,
    Wait,
    Reset,
    Clr(Operand),
    Asl(Operand),
    Jmp(Operand),
    Swab(Operand),
    Mov(Operand, Operand),
    Cmp(Operand, Operand),
    Bit(Operand, Operand),
    Invalid(u16),
}

impl Instruction {
    fn clr(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Clr(dst)
    }

    fn asl(opcode: u16) -> Self {
        let operand = Operand::from_0_5(opcode);
        Self::Asl(operand)
    }

    fn jmp(opcode: u16) -> Self {
        let src = Operand::from_0_5(opcode);
        Self::Jmp(src)
    }

    fn swab(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Swab(dst)
    }

    fn mov(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Mov(src, dst)
    }

    fn cmp(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Cmp(src, dst)
    }

    fn bit(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Bit(src, dst)
    }

    fn disassemble(&self) -> String {
        use Instruction::*;
        match self {
            Halt => "HALT".into(),
            Wait => "WAIT".into(),
            Reset => "RESET".into(),
            Clr(dst) => format!("CLR\t{dst}"),
            Asl(operand) => format!("ASL\t{operand}"),
            Jmp(src) => format!("JMP\t{src}"),
            Swab(dst) => format!("SWAB\t{dst}"),
            Mov(src, dst) => format!("MOV\t{src}, {dst}"),
            Cmp(src, dst) => format!("CMP\t{src}, {dst}"),
            Bit(src, dst) => format!("BIT\t{src}, {dst}"),
            Invalid(opcode) => format!("Invalid opcode {opcode:#08o}"),
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
            opcode @ 0o005000..=0o005077 => Self::clr(opcode),
            opcode @ 0o006300..=0o006377 => Self::asl(opcode),
            opcode @ 0o000100..=0o000177 => Self::jmp(opcode),
            opcode @ 0o000300..=0o000377 => Self::swab(opcode),
            opcode @ 0o010000..=0o017777 => Self::mov(opcode),
            opcode @ 0o020000..=0o027777 => Self::cmp(opcode),
            opcode @ 0o030000..=0o037777 => Self::bit(opcode),
            other => Instruction::Invalid(other),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.disassemble().fmt(f)
    }
}
