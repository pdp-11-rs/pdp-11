use std::fmt;
use std::ops;

pub use insns::Instruction;
pub use psw::{Flags::*, ProcessorStatusWord};
pub use ram::Byte;
pub use ram::Ram;
pub use ram::Word;
pub use register::Registers;
pub use register::{Register, Register::*};

mod bootrom;
mod impls;
mod insns;
mod psw;
mod ram;
mod register;

#[derive(Debug, Default)]
pub struct Cpu {
    halt: bool,
    registers: Registers,
    psw: ProcessorStatusWord,
    ram: Ram,
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
    pub fn new() -> Self {
        let mut this = Self::default();
        this.reset();
        this
    }

    pub fn run(mut self) {
        while !self.halt {
            let opcode = self.next_opcode();
            self.execute(opcode);
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
            Asl(operand) => self.asl(operand),
            Jmp(src) => self.jmp(src),
            Swab(dst) => self.swab(dst),
            Mov(src, dst) => self.mov(src, dst),
            Cmp(src, dst) => self.cmp(src, dst),
            Bit(src, dst) => self.bit(src, dst),
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
        *self = Self::default();
        self.psw.reset();
        self.bootrom();
        // let data = Word::from(0o010102);
        // let address = Word::from(0).address();
        // self.ram.store(address, data);
    }

    fn asl(&mut self, operand: Operand) {
        let word = self.word(operand).as_u16() << 1;
        *self.word_mut(operand) = word.into();
    }

    fn jmp(&mut self, src: Operand) {
        todo!("JMP")
    }

    fn swab(&mut self, dst: Operand) {
        self.word_mut(dst).swab();
    }

    pub fn mov(&mut self, src: Operand, dst: Operand) {
        let word = *self.word(src);
        *self.word_mut(dst) = word;
        self.psw[N] = word.is_negative();
        self.psw[Z] = word.is_zero();
        self.psw[V] = false;
    }

    pub fn cmp(&mut self, src: Operand, dst: Operand) {
        let src = *self.word(src);
        let dst = *self.word(dst);
        let cmp = src - dst;
        self.psw[Z] = cmp.is_zero();
        self.psw[N] = cmp.is_negative();
        // self.psw[V] = xxx;
        // self.psw[C] = xxx;
    }

    pub fn bit(&mut self, src: Operand, dst: Operand) {
        let src = *self.word(src);
        let dst = *self.word(dst);
        let bit = src & dst;
        self.psw[Z] = bit.is_zero();
        self.psw[N] = bit.is_negative();
        self.psw[V] = false;
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

pub trait MemoryAcceess: Into<Word> + From<Word> + Into<Word> + fmt::Debug + fmt::Octal {
    const SIZE: usize;
    type LittleEndian;

    fn from_le_bytes(bytes: &[u8]) -> Self;
    fn to_le(&self) -> Self::LittleEndian;
    fn as_le_bytes(&self) -> &[u8];
    fn is_zero(&self) -> bool;
    fn is_negative(&self) -> bool;
}
