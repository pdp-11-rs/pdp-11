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
    Tst(Operand),
    Mov(Operand, Operand),
    Cmp(Operand, Operand),
    Bit(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    // Branch instructions
    Br(Offset),  // Branch (unconditional)
    Bne(Offset), // Branch if Not Equal (Z=0)
    Beq(Offset), // Branch if Equal (Z=1)
    Bpl(Offset), // Branch if Plus (N=0)
    Bmi(Offset), // Branch if Minus (N=1)
    Bvc(Offset), // Branch if oVerflow Clear (V=0)
    Bvs(Offset), // Branch if oVerflow Set (V=1)
    Bcc(Offset), // Branch if Carry Clear (C=0)
    Bcs(Offset), // Branch if Carry Set (C=1)
    Bge(Offset), // Branch if Greater or Equal (N xor V = 0)
    Blt(Offset), // Branch if Less Than (N xor V = 1)
    Bgt(Offset), // Branch if Greater Than (Z or (N xor V) = 0)
    Ble(Offset), // Branch if Less or Equal (Z or (N xor V) = 1)
    Tstb(Operand),
    Jsr(Register, Operand), // Jump to Subroutine
    Rts(Register),          // Return from Subroutine
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

    fn tst(opcode: u16) -> Self {
        let src = Operand::from_0_5(opcode);
        Self::Tst(src)
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

    fn add(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Add(src, dst)
    }

    fn sub(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Sub(src, dst)
    }

    fn branch_offset(opcode: u16) -> Offset {
        let offset = opcode.to_le_bytes()[0] as i8;
        Offset(offset)
    }

    fn br(opcode: u16) -> Self {
        Self::Br(Self::branch_offset(opcode))
    }

    fn bne(opcode: u16) -> Self {
        Self::Bne(Self::branch_offset(opcode))
    }

    fn beq(opcode: u16) -> Self {
        Self::Beq(Self::branch_offset(opcode))
    }

    fn bpl(opcode: u16) -> Self {
        Self::Bpl(Self::branch_offset(opcode))
    }

    fn bmi(opcode: u16) -> Self {
        Self::Bmi(Self::branch_offset(opcode))
    }

    fn bvc(opcode: u16) -> Self {
        Self::Bvc(Self::branch_offset(opcode))
    }

    fn bvs(opcode: u16) -> Self {
        Self::Bvs(Self::branch_offset(opcode))
    }

    fn bcc(opcode: u16) -> Self {
        Self::Bcc(Self::branch_offset(opcode))
    }

    fn bcs(opcode: u16) -> Self {
        Self::Bcs(Self::branch_offset(opcode))
    }

    fn bge(opcode: u16) -> Self {
        Self::Bge(Self::branch_offset(opcode))
    }

    fn blt(opcode: u16) -> Self {
        Self::Blt(Self::branch_offset(opcode))
    }

    fn bgt(opcode: u16) -> Self {
        Self::Bgt(Self::branch_offset(opcode))
    }

    fn ble(opcode: u16) -> Self {
        Self::Ble(Self::branch_offset(opcode))
    }

    fn tstb(opcode: u16) -> Self {
        let src = Operand::from_0_5(opcode);
        Self::Tstb(src)
    }

    fn jsr(opcode: u16) -> Self {
        let register = Register::from((opcode >> 6) & 0o7);
        let dst = Operand::from_0_5(opcode);
        Self::Jsr(register, dst)
    }

    fn rts(opcode: u16) -> Self {
        let register = Register::from(opcode & 0o7);
        Self::Rts(register)
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
            Tst(src) => format!("TST\t{src}"),
            Mov(src, dst) => format!("MOV\t{src}, {dst}"),
            Cmp(src, dst) => format!("CMP\t{src}, {dst}"),
            Bit(src, dst) => format!("BIT\t{src}, {dst}"),
            Add(src, dst) => format!("ADD\t{src}, {dst}"),
            Sub(src, dst) => format!("SUB\t{src}, {dst}"),
            Br(offset) => format!("BR\t{offset}"),
            Bne(offset) => format!("BNE\t{offset}"),
            Beq(offset) => format!("BEQ\t{offset}"),
            Bpl(offset) => format!("BPL\t{offset}"),
            Bmi(offset) => format!("BMI\t{offset}"),
            Bvc(offset) => format!("BVC\t{offset}"),
            Bvs(offset) => format!("BVS\t{offset}"),
            Bcc(offset) => format!("BCC\t{offset}"),
            Bcs(offset) => format!("BCS\t{offset}"),
            Bge(offset) => format!("BGE\t{offset}"),
            Blt(offset) => format!("BLT\t{offset}"),
            Bgt(offset) => format!("BGT\t{offset}"),
            Ble(offset) => format!("BLE\t{offset}"),
            Tstb(src) => format!("TSTB\t{src}"),
            Jsr(register, dst) => format!("JSR\t{register}, {dst}"),
            Rts(register) => format!("RTS\t{register}"),
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
            opcode @ 0o005700..=0o005777 => Self::tst(opcode),
            opcode @ 0o010000..=0o017777 => Self::mov(opcode),
            opcode @ 0o020000..=0o027777 => Self::cmp(opcode),
            opcode @ 0o030000..=0o037777 => Self::bit(opcode),
            opcode @ 0o060000..=0o067777 => Self::add(opcode),
            opcode @ 0o160000..=0o167777 => Self::sub(opcode),
            // Branch instructions (all use low 8 bits as signed offset)
            opcode @ 0o000400..=0o000777 => Self::br(opcode),
            opcode @ 0o001000..=0o001377 => Self::bne(opcode),
            opcode @ 0o001400..=0o001777 => Self::beq(opcode),
            opcode @ 0o100000..=0o100377 => Self::bpl(opcode),
            opcode @ 0o100400..=0o100777 => Self::bmi(opcode),
            opcode @ 0o102000..=0o102377 => Self::bvc(opcode),
            opcode @ 0o102400..=0o102777 => Self::bvs(opcode),
            opcode @ 0o103000..=0o103377 => Self::bcc(opcode),
            opcode @ 0o103400..=0o103777 => Self::bcs(opcode),
            opcode @ 0o002000..=0o002377 => Self::bge(opcode),
            opcode @ 0o002400..=0o002777 => Self::blt(opcode),
            opcode @ 0o003000..=0o003377 => Self::bgt(opcode),
            opcode @ 0o003400..=0o003777 => Self::ble(opcode),
            opcode @ 0o105700..=0o105777 => Self::tstb(opcode),
            opcode @ 0o004000..=0o004777 => Self::jsr(opcode),
            opcode @ 0o000200..=0o000207 => Self::rts(opcode),
            other => Instruction::Invalid(other),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.disassemble().fmt(f)
    }
}
