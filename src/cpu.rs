use std::fmt;
use std::ops;

pub use insns::Instruction;
pub use ram::Byte;
pub use ram::Ram;
pub use ram::Word;
pub use register::Registers;
pub use register::{Register, Register::*};

mod insns;
mod ram;
mod register;

mod impls;

#[derive(Debug, Default)]
pub struct Cpu {
    halt: bool,
    registers: Registers,
    psw: ProcessorStatusWord,
    ram: Ram,
}

#[derive(Debug, Default)]
pub struct ProcessorStatusWord {
    carry: bool,
    overflow: bool,
    zero: bool,
    negative: bool,
    trap: bool,
    ipl: u8,
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
        self.load(Source::pc())
    }

    fn execute(&mut self, opcode: Word) {
        use Instruction::*;
        let instruction = Instruction::from(opcode);
        println!("Executing {opcode:#08o} {instruction}");

        match instruction {
            Halt => self.halt(),
            Wait => self.wait(),
            Reset => self.reset(),
            Mov(src, dst) => self.mov(src, dst),
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
        let data = Word::from(0o010102);
        let address = Word::from(0).address();
        self.ram.store(address, data);
    }

    pub fn mov(&mut self, src: Source, dst: Destination) {
        let word: Word = self.load(src);
        self.store(dst, word);
    }
}

#[derive(Debug)]
pub struct Source {
    mode: RegisterAddressingMode,
    register: Register,
}

#[derive(Debug)]
pub struct Destination {
    mode: RegisterAddressingMode,
    register: Register,
}

impl Source {
    pub fn from_double_operand(opcode: u16) -> Self {
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

impl Destination {
    pub fn from_double_operand(opcode: u16) -> Self {
        let mode = RegisterAddressingMode::from((opcode & 0o000070) >> 3);
        let register = Register::from(opcode & 0o000007);
        Self { mode, register }
    }
}

impl fmt::Display for Source {
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

impl fmt::Display for Destination {
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
}
