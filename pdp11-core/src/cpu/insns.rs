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
    Bic(Operand, Operand), // Bit Clear
    Bis(Operand, Operand), // Bit Set
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
    // Single operand instructions
    Com(Operand), // Complement (one's complement)
    Inc(Operand), // Increment
    Dec(Operand), // Decrement
    Neg(Operand), // Negate (two's complement)
    Adc(Operand), // Add Carry
    Sbc(Operand), // Subtract Carry
    Ror(Operand), // Rotate Right
    Rol(Operand), // Rotate Left
    Asr(Operand), // Arithmetic Shift Right
    // Byte single operand instructions
    Clrb(Operand), // Clear Byte
    Comb(Operand), // Complement Byte
    Incb(Operand), // Increment Byte
    Decb(Operand), // Decrement Byte
    Negb(Operand), // Negate Byte
    Adcb(Operand), // Add Carry Byte
    Sbcb(Operand), // Subtract Carry Byte
    Rorb(Operand), // Rotate Right Byte
    Rolb(Operand), // Rotate Left Byte
    Asrb(Operand), // Arithmetic Shift Right Byte
    // Double operand byte instructions
    Movb(Operand, Operand), // Move Byte
    Cmpb(Operand, Operand), // Compare Byte
    Bitb(Operand, Operand), // Bit Test Byte
    Bicb(Operand, Operand), // Bit Clear Byte
    Bisb(Operand, Operand), // Bit Set Byte
    Jsr(Register, Operand), // Jump to Subroutine
    Rts(Register),          // Return from Subroutine
    Rti,                    // Return from Interrupt
    // PSW flag manipulation instructions
    Nop, // No Operation
    Clc, // Clear Carry
    Sec, // Set Carry
    Clv, // Clear Overflow
    Sev, // Set Overflow
    Clz, // Clear Zero
    Sez, // Set Zero
    Cln, // Clear Negative
    Sen, // Set Negative
    Ccc, // Clear All Condition Codes
    Scc, // Set All Condition Codes
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

    fn bic(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Bic(src, dst)
    }

    fn bis(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Bis(src, dst)
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

    fn com(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Com(dst)
    }

    fn inc(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Inc(dst)
    }

    fn dec(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Dec(dst)
    }

    fn neg(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Neg(dst)
    }

    fn adc(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Adc(dst)
    }

    fn sbc(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Sbc(dst)
    }

    fn ror(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Ror(dst)
    }

    fn rol(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Rol(dst)
    }

    fn asr(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Asr(dst)
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

    fn rti() -> Self {
        Self::Rti
    }

    // Byte instruction decoders
    fn clrb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Clrb(dst)
    }

    fn comb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Comb(dst)
    }

    fn incb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Incb(dst)
    }

    fn decb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Decb(dst)
    }

    fn negb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Negb(dst)
    }

    fn adcb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Adcb(dst)
    }

    fn sbcb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Sbcb(dst)
    }

    fn rorb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Rorb(dst)
    }

    fn rolb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Rolb(dst)
    }

    fn asrb(opcode: u16) -> Self {
        let dst = Operand::from_0_5(opcode);
        Self::Asrb(dst)
    }

    fn movb(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Movb(src, dst)
    }

    fn cmpb(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Cmpb(src, dst)
    }

    fn bitb(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Bitb(src, dst)
    }

    fn bicb(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Bicb(src, dst)
    }

    fn bisb(opcode: u16) -> Self {
        let src = Operand::from_6_11(opcode);
        let dst = Operand::from_0_5(opcode);
        Self::Bisb(src, dst)
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
            Bic(src, dst) => format!("BIC\t{src}, {dst}"),
            Bis(src, dst) => format!("BIS\t{src}, {dst}"),
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
            Com(dst) => format!("COM\t{dst}"),
            Inc(dst) => format!("INC\t{dst}"),
            Dec(dst) => format!("DEC\t{dst}"),
            Neg(dst) => format!("NEG\t{dst}"),
            Adc(dst) => format!("ADC\t{dst}"),
            Sbc(dst) => format!("SBC\t{dst}"),
            Ror(dst) => format!("ROR\t{dst}"),
            Rol(dst) => format!("ROL\t{dst}"),
            Asr(dst) => format!("ASR\t{dst}"),
            Clrb(dst) => format!("CLRB\t{dst}"),
            Comb(dst) => format!("COMB\t{dst}"),
            Incb(dst) => format!("INCB\t{dst}"),
            Decb(dst) => format!("DECB\t{dst}"),
            Negb(dst) => format!("NEGB\t{dst}"),
            Adcb(dst) => format!("ADCB\t{dst}"),
            Sbcb(dst) => format!("SBCB\t{dst}"),
            Rorb(dst) => format!("RORB\t{dst}"),
            Rolb(dst) => format!("ROLB\t{dst}"),
            Asrb(dst) => format!("ASRB\t{dst}"),
            Movb(src, dst) => format!("MOVB\t{src}, {dst}"),
            Cmpb(src, dst) => format!("CMPB\t{src}, {dst}"),
            Bitb(src, dst) => format!("BITB\t{src}, {dst}"),
            Bicb(src, dst) => format!("BICB\t{src}, {dst}"),
            Bisb(src, dst) => format!("BISB\t{src}, {dst}"),
            Jsr(register, dst) => format!("JSR\t{register}, {dst}"),
            Rts(register) => format!("RTS\t{register}"),
            Rti => "RTI".into(),
            Nop => "NOP".into(),
            Clc => "CLC".into(),
            Sec => "SEC".into(),
            Clv => "CLV".into(),
            Sev => "SEV".into(),
            Clz => "CLZ".into(),
            Sez => "SEZ".into(),
            Cln => "CLN".into(),
            Sen => "SEN".into(),
            Ccc => "CCC".into(),
            Scc => "SCC".into(),
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
            opcode @ 0o040000..=0o047777 => Self::bic(opcode),
            opcode @ 0o050000..=0o057777 => Self::bis(opcode),
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
            opcode @ 0o005100..=0o005177 => Self::com(opcode),
            opcode @ 0o005200..=0o005277 => Self::inc(opcode),
            opcode @ 0o005300..=0o005377 => Self::dec(opcode),
            opcode @ 0o005400..=0o005477 => Self::neg(opcode),
            opcode @ 0o005500..=0o005577 => Self::adc(opcode),
            opcode @ 0o005600..=0o005677 => Self::sbc(opcode),
            opcode @ 0o006000..=0o006077 => Self::ror(opcode),
            opcode @ 0o006100..=0o006177 => Self::rol(opcode),
            opcode @ 0o006200..=0o006277 => Self::asr(opcode),
            // Byte single operand instructions
            opcode @ 0o105000..=0o105077 => Self::clrb(opcode),
            opcode @ 0o105100..=0o105177 => Self::comb(opcode),
            opcode @ 0o105200..=0o105277 => Self::incb(opcode),
            opcode @ 0o105300..=0o105377 => Self::decb(opcode),
            opcode @ 0o105400..=0o105477 => Self::negb(opcode),
            opcode @ 0o105500..=0o105577 => Self::adcb(opcode),
            opcode @ 0o105600..=0o105677 => Self::sbcb(opcode),
            opcode @ 0o106000..=0o106077 => Self::rorb(opcode),
            opcode @ 0o106100..=0o106177 => Self::rolb(opcode),
            opcode @ 0o106200..=0o106277 => Self::asrb(opcode),
            // Byte double operand instructions
            opcode @ 0o110000..=0o117777 => Self::movb(opcode),
            opcode @ 0o120000..=0o127777 => Self::cmpb(opcode),
            opcode @ 0o130000..=0o137777 => Self::bitb(opcode),
            opcode @ 0o140000..=0o147777 => Self::bicb(opcode),
            opcode @ 0o150000..=0o157777 => Self::bisb(opcode),
            opcode @ 0o004000..=0o004777 => Self::jsr(opcode),
            opcode @ 0o000200..=0o000207 => Self::rts(opcode),
            0o000002 => Self::rti(),
            0o000240 => Nop,
            0o000241 => Clc,
            0o000261 => Sec,
            0o000242 => Clv,
            0o000262 => Sev,
            0o000244 => Clz,
            0o000264 => Sez,
            0o000250 => Cln,
            0o000270 => Sen,
            0o000257 => Ccc,
            0o000277 => Scc,
            other => Instruction::Invalid(other),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.disassemble().fmt(f)
    }
}
