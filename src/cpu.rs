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
        let rk = rk::Rk::with_image(rk)?;
        let core = Self {
            halt: false,
            registers: Registers::default(),
            psw: ProcessorStatusWord::default(),
            ram: Ram::default(),
            rk,
        };

        Ok(core)
    }

    pub fn poweron(mut self) {
        self.reset();
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
            Clr(dst) => self.clr(dst),
            Asl(operand) => self.asl(operand),
            Jmp(src) => self.jmp(src),
            Swab(dst) => self.swab(dst),
            Tst(src) => self.tst(src),
            Mov(src, dst) => self.mov(src, dst),
            Cmp(src, dst) => self.cmp(src, dst),
            Bit(src, dst) => self.bit(src, dst),
            Bpl(offset) => self.bpl(offset),
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
        let effective_addr = self.effective_address(src);
        self.registers[PC] = effective_addr;
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
        self.psw[Z] = tst.is_negative();
        self.psw[N] = tst.is_negative();
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn mov(&mut self, src: Operand, dst: Operand) {
        let word = *self.word(src);
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
        // self.psw[V] = xxx;
        // self.psw[C] = xxx;
    }

    fn bit(&mut self, src: Operand, dst: Operand) {
        let src = *self.word(src);
        let dst = *self.word(dst);
        let bit = src & dst;
        self.psw[Z] = bit.is_zero();
        self.psw[N] = bit.is_negative();
        self.psw[V] = false;
    }

    fn bpl(&mut self, offset: Offset) {
        let positive = offset.0.is_positive();
        let offset = (offset.0.abs() * 2) as u8;
        if positive {
            self.registers[PC] += offset;
        } else {
            self.registers[PC] -= offset;
        }
    }

    fn tstb(&mut self, src: Operand) {
        let tstb = *self.byte(src);
        self.psw[Z] = tstb.is_negative();
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
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Helper to create a test CPU with a temporary RK image
    fn create_test_cpu() -> Cpu {
        // Create a temporary file for the RK disk image
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write a minimal disk image (at least 1 block = 512 bytes)
        temp_file.write_all(&[0u8; 512]).unwrap();
        temp_file.flush().unwrap();

        // Create CPU with the temporary RK image
        Cpu::new(temp_file.path()).unwrap()
    }

    #[test]
    fn test_jmp_register_deferred() {
        let mut cpu = create_test_cpu();
        // Set up R1 to point to address 0o1000
        cpu.registers[R1] = 0o1000.into();

        // JMP (R1) - opcode 0o000110 (mode=1, reg=1)
        let operand = Operand {
            mode: RegisterAddressingMode::RegisterDeferred,
            register: R1,
        };

        cpu.jmp(operand);

        // PC should now be 0o1000
        assert_eq!(cpu.registers[PC], 0o1000.into());
    }

    #[test]
    fn test_jmp_autoincrement() {
        let mut cpu = create_test_cpu();
        // Set up R2 to point to a memory location containing the target address
        cpu.registers[R2] = 0o2000.into();
        // Store target address 0o5000 at memory location 0o2000
        let addr = Address::<Word>::from_u16(0o2000);
        *cpu.ram.word_mut(addr) = 0o5000.into();

        // JMP (R2)+ - opcode 0o000122 (mode=2, reg=2)
        let operand = Operand {
            mode: RegisterAddressingMode::Autoincrement,
            register: R2,
        };

        cpu.jmp(operand);

        // PC should be set to the value read from memory
        assert_eq!(cpu.registers[PC], 0o5000.into());
        // R2 should be incremented by 2
        assert_eq!(cpu.registers[R2], 0o2002.into());
    }

    #[test]
    fn test_jmp_autoincrement_deferred() {
        let mut cpu = create_test_cpu();
        // Set up R3 to point to a memory location
        cpu.registers[R3] = 0o3000.into();
        // Store an address at 0o3000 that points to another address
        *cpu.ram.word_mut(Address::from_u16(0o3000)) = 0o4000.into();
        // Store the final target address at 0o4000
        *cpu.ram.word_mut(Address::from_u16(0o4000)) = 0o6000.into();

        // JMP @(R3)+ - opcode 0o000133 (mode=3, reg=3)
        let operand = Operand {
            mode: RegisterAddressingMode::AutoincrementDeferred,
            register: R3,
        };

        cpu.jmp(operand);

        // PC should be set to the final target
        assert_eq!(cpu.registers[PC], 0o6000.into());
        // R3 should be incremented
        assert_eq!(cpu.registers[R3], 0o3002.into());
    }

    #[test]
    fn test_jmp_autodecrement() {
        let mut cpu = create_test_cpu();
        // Set up R4 to point just after the memory location containing target
        cpu.registers[R4] = 0o1002.into();
        // Store target address at 0o1000
        *cpu.ram.word_mut(Address::from_u16(0o1000)) = 0o7000.into();

        // JMP -(R4) - opcode 0o000144 (mode=4, reg=4)
        let operand = Operand {
            mode: RegisterAddressingMode::Autodecrement,
            register: R4,
        };

        cpu.jmp(operand);

        // PC should be set to the target
        assert_eq!(cpu.registers[PC], 0o7000.into());
        // R4 should be decremented by 2
        assert_eq!(cpu.registers[R4], 0o1000.into());
    }

    #[test]
    #[ignore] // Index mode not yet implemented
    fn test_jmp_index() {
        let mut cpu = create_test_cpu();
        // Set up PC for index mode (it will be used to read the index value)
        cpu.registers[PC] = 0o1000.into();
        // Store index offset at 0o1000
        *cpu.ram.word_mut(Address::from_u16(0o1000)) = 0o100.into();
        // Set up R5 as base
        cpu.registers[R5] = 0o2000.into();

        // JMP X(R5) - opcode 0o000165 (mode=6, reg=5)
        let operand = Operand {
            mode: RegisterAddressingMode::Index,
            register: R5,
        };

        cpu.jmp(operand);

        // PC should be set to R5 + index = 0o2000 + 0o100 = 0o2100
        assert_eq!(cpu.registers[PC], 0o2100.into());
    }

    #[test]
    #[ignore] // Index mode not yet implemented
    fn test_jmp_pc_relative() {
        let mut cpu = create_test_cpu();
        // Set up PC
        cpu.registers[PC] = 0o1000.into();
        // Store offset at 0o1000 (PC-relative addressing)
        *cpu.ram.word_mut(Address::from_u16(0o1000)) = 0o500.into();

        // JMP X(PC) - PC-relative addressing
        let operand = Operand {
            mode: RegisterAddressingMode::Index,
            register: PC,
        };

        cpu.jmp(operand);

        // PC should be 0o1000 (old PC) + 2 (auto-increment) + 0o500 (offset) = 0o1502
        assert_eq!(cpu.registers[PC], 0o1502.into());
    }
}
