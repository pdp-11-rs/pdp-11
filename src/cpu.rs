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

#[derive(Debug, Default)]
pub struct Cpu {
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
        Self::default()
    }

    pub fn run(mut self) {
        loop {
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
        println!("Executing {instruction}");

        match instruction {
            Mov(src, dst) => self.mov(src, dst),
            Invalid(opcode) => eprintln!("Opcode {opcode:#08o} is not supported yet"),
        }
    }
}

impl Cpu {
    pub fn load<M>(&mut self, src: Source) -> M
    where
        M: MemoryAcceess + From<Word>,
    {
        use RegisterAddressingMode::*;

        let Source { mode, register } = src;

        match src.mode {
            Register => self.registers.get::<M>(register, mode).into(),
            RegisterDeferred => todo!("load deferred"),
            Autoincrement => {
                let address = self.registers.get::<M>(register, mode).address();
                self.ram.load(address)
            }
            AutoincrementDeferred => todo!("load autoincrement deferred"),
            Autodecrement => todo!("load autodecrement"),
            AutodecrementDeferred => todo!("load autodecrement deferred"),
            Index => todo!("load index"),
            IndexDeferred => todo!("load index deferred"),
        }
    }

    pub fn store<M>(&mut self, dst: Destination, data: M)
    where
        M: MemoryAcceess,
        Word: From<M>,
    {
        use RegisterAddressingMode::*;

        let Destination { mode, register } = dst;
        match mode {
            Register => {
                self.registers.set(register, mode, data);
            }
            RegisterDeferred => {
                let address = self.registers.get::<Word>(register, mode).address();
                self.ram.store(address, data);
            }
            Autoincrement => todo!("store Autoincrement"),
            AutoincrementDeferred => todo!("store  AutoincrementDeferred"),
            Autodecrement => todo!(),
            AutodecrementDeferred => todo!(),
            Index => todo!(),
            IndexDeferred => todo!(),
        };
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
        let mode = RegisterAddressingMode::from(opcode & 0o00_70_00 >> 9);
        let register = Register::from(opcode & 0o00_07_00 >> 6);
        Self { mode, register }
    }

    pub fn pc() -> Self {
        Self {
            mode: RegisterAddressingMode::Autoincrement,
            register: R7,
        }
    }
}

impl Destination {
    pub fn from_double_operand(opcode: u16) -> Self {
        let mode = RegisterAddressingMode::from(opcode & 0o00_00_70 >> 3);
        let register = Register::from(opcode & 0o00_00_07);
        Self { mode, register }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub trait MemoryAcceess: Into<Word> + From<Word> + std::fmt::Debug {
    const SIZE: usize;
    type LittleEndian;

    fn from_le_bytes(bytes: &[u8]) -> Self;
    fn to_le(&self) -> Self::LittleEndian;
    // fn to_le_bytes(&self) -> &[u8];
    fn as_le_bytes(&self) -> &[u8];
}
