use super::*;

pub use insns::Instruction;
pub use psw::{Flags::*, ProcessorStatusWord};
pub use ram::Address;
pub use ram::Byte;
pub use ram::Ram;
pub use ram::Word;
pub use register::Registers;
pub use register::{Register, Register::*};

mod bootrom;
mod console;
mod impls;
mod insns;
mod psw;
mod ram;
mod register;
mod rk;

#[derive(Debug)]
pub struct Cpu {
    halt: bool,
    registers: Registers,
    psw: ProcessorStatusWord,
    ram: Ram,
    rk: rk::Rk,
    console: console::Console,
    /// Temporary storage for memory-mapped I/O register reads
    /// This allows returning references to RK11 registers
    io_temp: Word,
}

#[derive(Clone, Copy, Debug)]
pub enum RegisterAddressingMode {
    Register,
    RegisterDeferred,
    Autoincrement,
    AutoincrementDeferred,
    Autodecrement,
    AutodecrementDeferred,
    Index,
    IndexDeferred,
}

impl From<u16> for RegisterAddressingMode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Register,
            1 => Self::RegisterDeferred,
            2 => Self::Autoincrement,
            3 => Self::AutoincrementDeferred,
            4 => Self::Autodecrement,
            5 => Self::AutodecrementDeferred,
            6 => Self::Index,
            7 => Self::IndexDeferred,
            other => panic!("Invalid register access mode {other:o}"),
        }
    }
}

// pub enum PcAddressingMode {
//     Immediate,
//     Absolute,
//     Relative,
//     RelativeDeferred,
// }

impl Cpu {
    pub fn new(rk: impl AsRef<Path>) -> io::Result<Self> {
        let mut rk = rk::Rk::with_image(rk)?;
        let console = console::Console::new();
        let mut ram = Ram::default();

        // Initialize peripheral registers in RAM
        rk.init_registers(&mut ram);
        console.init_registers(&mut ram);

        let core = Self {
            halt: false,
            registers: Registers::default(),
            psw: ProcessorStatusWord::default(),
            ram,
            rk,
            console,
            io_temp: Word::zero(),
        };

        Ok(core)
    }

    /// Create a CPU with an empty RK disk for testing purposes
    /// This avoids the need for temporary files in tests
    #[cfg(test)]
    pub fn for_testing() -> Self {
        let mut rk = rk::Rk::empty();
        let console = console::Console::new();
        let mut ram = Ram::default();

        // Initialize peripheral registers in RAM
        rk.init_registers(&mut ram);
        console.init_registers(&mut ram);

        Self {
            halt: false,
            registers: Registers::default(),
            psw: ProcessorStatusWord::default(),
            ram,
            rk,
            console,
            io_temp: Word::zero(),
        }
    }

    pub fn poweron(mut self) {
        self.reset();
        while !self.halt {
            let opcode = self.next_opcode();
            self.execute(opcode);
            // Check if any RK command was triggered
            self.rk.check_command(&mut self.ram);
        }
    }

    fn next_opcode(&mut self) -> Word {
        *self.word(Operand::pc())
    }

    fn execute(&mut self, opcode: Word) {
        use Instruction::*;
        let instruction = Instruction::from(opcode);
        println!("Executing {opcode:#08o}\t{instruction}");

        match instruction {
            Halt => self.halt(),
            Wait => self.wait(),
            Reset => self.reset(),
            Clr(dst) => self.clr(dst),
            Asl(operand) => self.asl(operand),
            Jmp(src) => self.jmp(src),
            Swab(dst) => self.swab(dst),
            Tst(src) => self.tst(src),
            Mov(src, dst) => self.mov(src, dst),
            Cmp(src, dst) => self.cmp(src, dst),
            Bit(src, dst) => self.bit(src, dst),
            Add(src, dst) => self.add(src, dst),
            Sub(src, dst) => self.sub(src, dst),
            Br(offset) => self.br(offset),
            Bne(offset) => self.bne(offset),
            Beq(offset) => self.beq(offset),
            Bpl(offset) => self.bpl(offset),
            Bmi(offset) => self.bmi(offset),
            Bvc(offset) => self.bvc(offset),
            Bvs(offset) => self.bvs(offset),
            Bcc(offset) => self.bcc(offset),
            Bcs(offset) => self.bcs(offset),
            Bge(offset) => self.bge(offset),
            Blt(offset) => self.blt(offset),
            Bgt(offset) => self.bgt(offset),
            Ble(offset) => self.ble(offset),
            Tstb(src) => self.tstb(src),
            Invalid(opcode) => eprintln!("Opcode {opcode:#08o} is not supported yet"),
        }
    }
}

impl Cpu {
    fn halt(&mut self) {
        self.halt = true;
    }

    fn wait(&mut self) {
        self.halt = true;
    }

    fn reset(&mut self) {
        self.halt = false;
        self.registers.reset();
        self.psw.reset();
        self.ram.reset();
        self.bootrom();
    }

    fn clr(&mut self, dst: Operand) {
        self.word_mut(dst).clear();
        self.psw[Z] = true;
        self.psw[N] = false;
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn asl(&mut self, operand: Operand) {
        let word = self.word(operand).as_u16() << 1;
        *self.word_mut(operand) = word.into();
    }

    /// JMP instruction: transfer control to the effective address
    fn jmp(&mut self, src: Operand) {
        use RegisterAddressingMode::*;

        // For most modes, JMP uses effective_address
        // But for Autoincrement/Autodecrement, we need to read the value from memory
        let target = match src.mode {
            Autoincrement | Autodecrement | AutoincrementDeferred | AutodecrementDeferred => {
                // For these modes, read the target address from memory
                *self.word(src)
            }
            _ => {
                // For other modes, use the effective address directly
                self.effective_address(src)
            }
        };

        self.registers[PC] = target;
    }

    fn swab(&mut self, dst: Operand) {
        self.word_mut(dst).swab();
        let word = *self.word(dst);
        self.psw[Z] = word.is_zero();
        self.psw[N] = word.is_negative();
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn tst(&mut self, src: Operand) {
        let tst = *self.word(src);
        self.psw[Z] = tst.is_zero();
        self.psw[N] = tst.is_negative();
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn mov(&mut self, src: Operand, dst: Operand) {
        let word = *self.word(src);

        // Check if destination is console I/O
        let dst_address = self.get_operand_address(dst);
        #[allow(clippy::collapsible_if)]
        if let Some(addr) = dst_address {
            if self.is_console_io(addr) {
                self.write_console(addr, word);
                self.psw[N] = word.is_negative();
                self.psw[Z] = word.is_zero();
                self.psw[V] = false;
                return;
            }
        }

        // Normal RAM write
        *self.word_mut(dst) = word;
        self.psw[N] = word.is_negative();
        self.psw[Z] = word.is_zero();
        self.psw[V] = false;
    }

    fn cmp(&mut self, src: Operand, dst: Operand) {
        let src = *self.word(src);
        let dst = *self.word(dst);
        let cmp = src - dst;
        self.psw[Z] = cmp.is_zero();
        self.psw[N] = cmp.is_negative();

        // CMP performs src - dst, so calculate flags using subtraction logic
        let src_u16 = src.as_u16();
        let dst_u16 = dst.as_u16();
        let (result_u16, borrow) = src_u16.overflowing_sub(dst_u16);
        self.psw[C] = borrow;

        // Overflow occurs when subtracting opposite signs produces result of wrong sign
        let src_sign = src_u16 & 0x8000 != 0;
        let dst_sign = dst_u16 & 0x8000 != 0;
        let result_sign = result_u16 & 0x8000 != 0;
        self.psw[V] = src_sign != dst_sign && src_sign != result_sign;
    }

    fn bit(&mut self, src: Operand, dst: Operand) {
        let src = *self.word(src);
        let dst = *self.word(dst);
        let bit = src & dst;
        self.psw[Z] = bit.is_zero();
        self.psw[N] = bit.is_negative();
        self.psw[V] = false;
    }

    fn add(&mut self, src: Operand, dst: Operand) {
        let src_val = *self.word(src);
        let dst_val = *self.word(dst);

        // Perform addition with overflow detection
        let src_u16 = src_val.as_u16();
        let dst_u16 = dst_val.as_u16();
        let (result_u16, carry) = dst_u16.overflowing_add(src_u16);
        let result = Word::from(result_u16);

        // Store result
        *self.word_mut(dst) = result;

        // Set flags
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = carry;

        // Overflow occurs when two numbers of same sign produce result of opposite sign
        let src_sign = src_u16 & 0x8000 != 0;
        let dst_sign = dst_u16 & 0x8000 != 0;
        let result_sign = result_u16 & 0x8000 != 0;
        self.psw[V] = src_sign == dst_sign && src_sign != result_sign;
    }

    fn sub(&mut self, src: Operand, dst: Operand) {
        let src_val = *self.word(src);
        let dst_val = *self.word(dst);

        // Perform subtraction with underflow detection
        let src_u16 = src_val.as_u16();
        let dst_u16 = dst_val.as_u16();
        let (result_u16, borrow) = dst_u16.overflowing_sub(src_u16);
        let result = Word::from(result_u16);

        // Store result
        *self.word_mut(dst) = result;

        // Set flags
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = borrow;

        // Overflow occurs when subtracting opposite signs produces result of wrong sign
        let src_sign = src_u16 & 0x8000 != 0;
        let dst_sign = dst_u16 & 0x8000 != 0;
        let result_sign = result_u16 & 0x8000 != 0;
        self.psw[V] = src_sign != dst_sign && dst_sign != result_sign;
    }

    fn branch(&mut self, offset: Offset) {
        // Offset is a signed byte offset in words (not bytes)
        // PC is already pointing to the next instruction
        let offset_words = offset.0 as i16 * 2;
        if offset_words >= 0 {
            self.registers[PC] += offset_words as u16;
        } else {
            self.registers[PC] -= offset_words.unsigned_abs();
        }
    }

    fn br(&mut self, offset: Offset) {
        // Branch unconditionally
        self.branch(offset);
    }

    fn bne(&mut self, offset: Offset) {
        // Branch if Not Equal (Z = 0)
        if !self.psw[Z] {
            self.branch(offset);
        }
    }

    fn beq(&mut self, offset: Offset) {
        // Branch if Equal (Z = 1)
        if self.psw[Z] {
            self.branch(offset);
        }
    }

    fn bpl(&mut self, offset: Offset) {
        // Branch if Plus (N = 0)
        if !self.psw[N] {
            self.branch(offset);
        }
    }

    fn bmi(&mut self, offset: Offset) {
        // Branch if Minus (N = 1)
        if self.psw[N] {
            self.branch(offset);
        }
    }

    fn bvc(&mut self, offset: Offset) {
        // Branch if oVerflow Clear (V = 0)
        if !self.psw[V] {
            self.branch(offset);
        }
    }

    fn bvs(&mut self, offset: Offset) {
        // Branch if oVerflow Set (V = 1)
        if self.psw[V] {
            self.branch(offset);
        }
    }

    fn bcc(&mut self, offset: Offset) {
        // Branch if Carry Clear (C = 0)
        if !self.psw[C] {
            self.branch(offset);
        }
    }

    fn bcs(&mut self, offset: Offset) {
        // Branch if Carry Set (C = 1)
        if self.psw[C] {
            self.branch(offset);
        }
    }

    fn bge(&mut self, offset: Offset) {
        // Branch if Greater or Equal (N xor V = 0)
        if self.psw[N] == self.psw[V] {
            self.branch(offset);
        }
    }

    fn blt(&mut self, offset: Offset) {
        // Branch if Less Than (N xor V = 1)
        if self.psw[N] != self.psw[V] {
            self.branch(offset);
        }
    }

    fn bgt(&mut self, offset: Offset) {
        // Branch if Greater Than (Z = 0 and (N xor V = 0))
        if !self.psw[Z] && self.psw[N] == self.psw[V] {
            self.branch(offset);
        }
    }

    fn ble(&mut self, offset: Offset) {
        // Branch if Less or Equal (Z = 1 or (N xor V = 1))
        if self.psw[Z] || self.psw[N] != self.psw[V] {
            self.branch(offset);
        }
    }

    fn tstb(&mut self, src: Operand) {
        let tstb = *self.byte(src);
        self.psw[Z] = tstb.is_zero();
        self.psw[N] = tstb.is_negative();
        self.psw[V] = false;
        self.psw[C] = false;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Operand {
    mode: RegisterAddressingMode,
    register: Register,
}

impl Operand {
    pub fn from_0_5(opcode: u16) -> Self {
        let mode = RegisterAddressingMode::from((opcode & 0o000070) >> 3);
        let register = Register::from(opcode & 0o000007);
        Self { mode, register }
    }

    pub fn from_6_11(opcode: u16) -> Self {
        let mode = RegisterAddressingMode::from((opcode & 0o007000) >> 9);
        let register = Register::from((opcode & 0o000700) >> 6);
        Self { mode, register }
    }

    pub fn pc() -> Self {
        Self {
            mode: RegisterAddressingMode::Autoincrement,
            register: PC,
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RegisterAddressingMode::*;

        let Self { mode, register } = self;
        match mode {
            Register => register.fmt(f),
            RegisterDeferred => format!("({register})").fmt(f),
            Autoincrement => format!("({register})+").fmt(f),
            AutoincrementDeferred => format!("@({register})+").fmt(f),
            Autodecrement => format!("-({register})").fmt(f),
            AutodecrementDeferred => format!("@-({register})").fmt(f),
            Index => format!("X({register})").fmt(f),
            IndexDeferred => format!("@X({register})").fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Offset(i8);

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!(".{:+}", self.0).fmt(f)
    }
}

pub trait MemoryAcceess: Into<Word> + From<Word> + Into<Word> + fmt::Debug + fmt::Octal {
    const SIZE: usize;
    type LittleEndian;

    fn from_le_bytes(bytes: &[u8]) -> Self;
    fn to_le(&self) -> Self::LittleEndian;
    fn as_le_bytes(&self) -> &[u8];
    fn is_zero(&self) -> bool;
    fn is_negative(&self) -> bool;
}

#[cfg(test)]
mod tests;
