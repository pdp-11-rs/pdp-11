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
        let rk = rk::Rk::with_image(rk)?;
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
        let rk = rk::Rk::empty();
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
            Bic(src, dst) => self.bic(src, dst),
            Bis(src, dst) => self.bis(src, dst),
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
            Com(dst) => self.com(dst),
            Inc(dst) => self.inc(dst),
            Dec(dst) => self.dec(dst),
            Neg(dst) => self.neg(dst),
            Adc(dst) => self.adc(dst),
            Sbc(dst) => self.sbc(dst),
            Ror(dst) => self.ror(dst),
            Rol(dst) => self.rol(dst),
            Asr(dst) => self.asr(dst),
            Clrb(dst) => self.clrb(dst),
            Comb(dst) => self.comb(dst),
            Incb(dst) => self.incb(dst),
            Decb(dst) => self.decb(dst),
            Negb(dst) => self.negb(dst),
            Adcb(dst) => self.adcb(dst),
            Sbcb(dst) => self.sbcb(dst),
            Rorb(dst) => self.rorb(dst),
            Rolb(dst) => self.rolb(dst),
            Asrb(dst) => self.asrb(dst),
            Movb(src, dst) => self.movb(src, dst),
            Cmpb(src, dst) => self.cmpb(src, dst),
            Bitb(src, dst) => self.bitb(src, dst),
            Bicb(src, dst) => self.bicb(src, dst),
            Bisb(src, dst) => self.bisb(src, dst),
            Jsr(register, dst) => self.jsr(register, dst),
            Rts(register) => self.rts(register),
            Nop => self.nop(),
            Clc => self.clc(),
            Sec => self.sec(),
            Clv => self.clv(),
            Sev => self.sev(),
            Clz => self.clz(),
            Sez => self.sez(),
            Cln => self.cln(),
            Sen => self.sen(),
            Ccc => self.ccc(),
            Scc => self.scc(),
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

    fn bic(&mut self, src: Operand, dst: Operand) {
        // BIC: Bit Clear - dst = dst & ~src
        let src = *self.word(src);
        let dst_val = *self.word(dst);
        let result = dst_val & !src;
        *self.word_mut(dst) = result;
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
    }

    fn bis(&mut self, src: Operand, dst: Operand) {
        // BIS: Bit Set - dst = dst | src
        let src = *self.word(src);
        let dst_val = *self.word(dst);
        let result = dst_val | src;
        *self.word_mut(dst) = result;
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
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

    fn jsr(&mut self, register: Register, dst: Operand) {
        // JSR: Jump to Subroutine
        // Push register onto stack via SP (R6)
        self.registers[SP] -= 2u16;
        let sp_addr = self.registers[SP].address::<Word>();
        self.ram[sp_addr] = self.registers[register];

        // Save PC in the register
        self.registers[register] = self.registers[PC];

        // Jump to destination - we need the effective address as a Word value
        // For most modes, this means the address itself (not the value stored there)
        use RegisterAddressingMode::*;
        let target = match dst.mode {
            Register => {
                // JSR to a register means jump to the value in that register
                self.registers[dst.register]
            }
            RegisterDeferred => {
                // JSR (Rn) means jump to the address stored in Rn
                self.registers[dst.register]
            }
            Autoincrement => {
                // JSR (Rn)+ means jump to address in Rn, then increment Rn
                let addr = self.registers[dst.register];
                self.registers[dst.register] += 2u16;
                addr
            }
            AutoincrementDeferred => {
                // JSR @(Rn)+ means jump to address pointed to by value in Rn, then increment Rn
                let addr_of_addr = self.registers[dst.register];
                self.registers[dst.register] += 2u16;
                self.ram[addr_of_addr.address::<Word>()]
            }
            Autodecrement => {
                // JSR -(Rn) means decrement Rn, then jump to address in Rn
                self.registers[dst.register] -= 2u16;
                self.registers[dst.register]
            }
            AutodecrementDeferred => {
                // JSR @-(Rn) means decrement Rn, jump to address pointed to by value in Rn
                self.registers[dst.register] -= 2u16;
                let addr_of_addr = self.registers[dst.register];
                self.ram[addr_of_addr.address::<Word>()]
            }
            Index => {
                // JSR X(Rn) means jump to address (Rn + X)
                let offset = *self.word(Operand::pc());
                self.registers[dst.register] + offset
            }
            IndexDeferred => {
                // JSR @X(Rn) means jump to address pointed to by (Rn + X)
                let offset = *self.word(Operand::pc());
                let addr_of_addr = (self.registers[dst.register] + offset).address::<Word>();
                self.ram[addr_of_addr]
            }
        };

        self.registers[PC] = target;
    }

    fn rts(&mut self, register: Register) {
        // RTS: Return from Subroutine
        // Load PC from register
        self.registers[PC] = self.registers[register];

        // Pop register from stack via SP (R6)
        let sp_addr = self.registers[SP].address::<Word>();
        self.registers[register] = self.ram[sp_addr];
        self.registers[SP] += 2u16;
    }

    fn com(&mut self, dst: Operand) {
        // COM: Complement (one's complement)
        let result = !*self.word(dst);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = false;
        self.psw[C] = true; // Always set
    }

    fn inc(&mut self, dst: Operand) {
        // INC: Increment
        let value = *self.word(dst);
        let result = value + Word::from_u16(1);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Word::from_u16(0o077777); // Overflow from max positive
    }

    fn dec(&mut self, dst: Operand) {
        // DEC: Decrement
        let value = *self.word(dst);
        let result = value - Word::from_u16(1);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Word::from_u16(0o100000); // Overflow from min negative
    }

    fn neg(&mut self, dst: Operand) {
        // NEG: Negate (two's complement)
        let value = *self.word(dst);
        let result = Word::from_u16(0u16.wrapping_sub(value.as_u16()));
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Word::from_u16(0o100000); // Overflow from min negative
        self.psw[C] = !result.is_zero(); // Clear if result is zero
    }

    fn adc(&mut self, dst: Operand) {
        // ADC: Add Carry
        let value = *self.word(dst);
        let carry = if self.psw[C] { 1u16 } else { 0u16 };
        let result = value + Word::from_u16(carry);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=077777 (max positive)
        self.psw[V] = carry == 1 && value == Word::from_u16(0o077777);
        // Carry if value=177777 and carry=1
        self.psw[C] = carry == 1 && value == Word::from_u16(0o177777);
    }

    fn sbc(&mut self, dst: Operand) {
        // SBC: Subtract Carry
        let value = *self.word(dst);
        let carry = if self.psw[C] { 1u16 } else { 0u16 };
        let result = value - Word::from_u16(carry);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=100000 (min negative)
        self.psw[V] = carry == 1 && value == Word::from_u16(0o100000);
        // Carry set if result borrows (value was 0 and carry was 1)
        self.psw[C] = carry == 1 && value.is_zero();
    }

    fn ror(&mut self, dst: Operand) {
        // ROR: Rotate Right through carry
        let value = *self.word(dst);
        let old_carry = if self.psw[C] { 1u16 } else { 0u16 };
        let new_carry = value.as_u16() & 1;
        let result = Word::from_u16((value.as_u16() >> 1) | (old_carry << 15));
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn rol(&mut self, dst: Operand) {
        // ROL: Rotate Left through carry
        let value = *self.word(dst);
        let old_carry = if self.psw[C] { 1u16 } else { 0u16 };
        let new_carry = (value.as_u16() >> 15) & 1;
        let result = Word::from_u16((value.as_u16() << 1) | old_carry);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn asr(&mut self, dst: Operand) {
        // ASR: Arithmetic Shift Right (sign-extend)
        let value = *self.word(dst);
        let sign_bit = value.as_u16() & 0o100000;
        let new_carry = value.as_u16() & 1;
        let result = Word::from_u16((value.as_u16() >> 1) | sign_bit);
        *self.word_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn nop(&mut self) {
        // NOP: No operation
    }

    fn clc(&mut self) {
        // CLC: Clear Carry
        self.psw[C] = false;
    }

    fn sec(&mut self) {
        // SEC: Set Carry
        self.psw[C] = true;
    }

    fn clv(&mut self) {
        // CLV: Clear Overflow
        self.psw[V] = false;
    }

    fn sev(&mut self) {
        // SEV: Set Overflow
        self.psw[V] = true;
    }

    fn clz(&mut self) {
        // CLZ: Clear Zero
        self.psw[Z] = false;
    }

    fn sez(&mut self) {
        // SEZ: Set Zero
        self.psw[Z] = true;
    }

    fn cln(&mut self) {
        // CLN: Clear Negative
        self.psw[N] = false;
    }

    fn sen(&mut self) {
        // SEN: Set Negative
        self.psw[N] = true;
    }

    fn ccc(&mut self) {
        // CCC: Clear All Condition Codes
        self.psw[N] = false;
        self.psw[Z] = false;
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn scc(&mut self) {
        // SCC: Set All Condition Codes
        self.psw[N] = true;
        self.psw[Z] = true;
        self.psw[V] = true;
        self.psw[C] = true;
    }

    // Byte instructions

    fn clrb(&mut self, dst: Operand) {
        self.byte_mut(dst).clear();
        self.psw[Z] = true;
        self.psw[N] = false;
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn comb(&mut self, dst: Operand) {
        // COMB: Complement byte (one's complement)
        let result = !*self.byte(dst);
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = false;
        self.psw[C] = true; // Always set
    }

    fn incb(&mut self, dst: Operand) {
        // INCB: Increment byte
        let value = *self.byte(dst);
        let result = value + 1u8.into();
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == 0o177u8.into(); // Overflow from max positive
    }

    fn decb(&mut self, dst: Operand) {
        // DECB: Decrement byte
        let value = *self.byte(dst);
        let result = value - 1u8.into();
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == 0o200u8.into(); // Overflow from min negative
    }

    fn negb(&mut self, dst: Operand) {
        // NEGB: Negate byte (two's complement)
        let value = *self.byte(dst);
        let result = Byte::from(0u8.wrapping_sub(value.as_u8()));
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == 0o200u8.into(); // Overflow from min negative
        self.psw[C] = !result.is_zero(); // Clear if result is zero
    }

    fn adcb(&mut self, dst: Operand) {
        // ADCB: Add Carry to byte
        let value = *self.byte(dst);
        let carry = if self.psw[C] { 1u8 } else { 0u8 };
        let result = value + carry.into();
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=0177 (max positive)
        self.psw[V] = carry == 1 && value == 0o177u8.into();
        // Carry if value=0377 and carry=1
        self.psw[C] = carry == 1 && value == 0o377u8.into();
    }

    fn sbcb(&mut self, dst: Operand) {
        // SBCB: Subtract Carry from byte
        let value = *self.byte(dst);
        let carry = if self.psw[C] { 1u8 } else { 0u8 };
        let result = value - carry.into();
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=0200 (min negative)
        self.psw[V] = carry == 1 && value == 0o200u8.into();
        // Carry set if result borrows (value was 0 and carry was 1)
        self.psw[C] = carry == 1 && value.is_zero();
    }

    fn rorb(&mut self, dst: Operand) {
        // RORB: Rotate Right byte through carry
        let value = *self.byte(dst);
        let old_carry = if self.psw[C] { 1u8 } else { 0u8 };
        let new_carry = value.as_u8() & 1;
        let result = Byte::from((value.as_u8() >> 1) | (old_carry << 7));
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn rolb(&mut self, dst: Operand) {
        // ROLB: Rotate Left byte through carry
        let value = *self.byte(dst);
        let old_carry = if self.psw[C] { 1u8 } else { 0u8 };
        let new_carry = (value.as_u8() >> 7) & 1;
        let result = Byte::from((value.as_u8() << 1) | old_carry);
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn asrb(&mut self, dst: Operand) {
        // ASRB: Arithmetic Shift Right byte (sign-extend)
        let value = *self.byte(dst);
        let sign_bit = value.as_u8() & 0o200;
        let new_carry = value.as_u8() & 1;
        let result = Byte::from((value.as_u8() >> 1) | sign_bit);
        *self.byte_mut(dst) = result;
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn movb(&mut self, src: Operand, dst: Operand) {
        let byte = *self.byte(src);
        *self.byte_mut(dst) = byte;
        self.psw[N] = byte.is_negative();
        self.psw[Z] = byte.is_zero();
        self.psw[V] = false;
    }

    fn cmpb(&mut self, src: Operand, dst: Operand) {
        let src = *self.byte(src);
        let dst = *self.byte(dst);
        let cmp = src - dst;
        self.psw[Z] = cmp.is_zero();
        self.psw[N] = cmp.is_negative();

        // CMPB performs src - dst, so calculate flags using subtraction logic
        let src_u8 = src.as_u8();
        let dst_u8 = dst.as_u8();
        let (result_u8, borrow) = src_u8.overflowing_sub(dst_u8);
        self.psw[C] = borrow;

        // Overflow occurs when subtracting opposite signs produces result of wrong sign
        let src_sign = src_u8 & 0x80 != 0;
        let dst_sign = dst_u8 & 0x80 != 0;
        let result_sign = result_u8 & 0x80 != 0;
        self.psw[V] = src_sign != dst_sign && src_sign != result_sign;
    }

    fn bitb(&mut self, src: Operand, dst: Operand) {
        let src = *self.byte(src);
        let dst = *self.byte(dst);
        let bit = src & dst;
        self.psw[Z] = bit.is_zero();
        self.psw[N] = bit.is_negative();
        self.psw[V] = false;
    }

    fn bicb(&mut self, src: Operand, dst: Operand) {
        // BICB: Bit Clear Byte - dst = dst & ~src
        let src = *self.byte(src);
        let dst_val = *self.byte(dst);
        let result = dst_val & !src;
        *self.byte_mut(dst) = result;
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
    }

    fn bisb(&mut self, src: Operand, dst: Operand) {
        // BISB: Bit Set Byte - dst = dst | src
        let src = *self.byte(src);
        let dst_val = *self.byte(dst);
        let result = dst_val | src;
        *self.byte_mut(dst) = result;
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
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
