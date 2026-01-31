use super::*;

pub use insns::Instruction;
pub use pdp11_common::Register::{self, *};
pub use psw::{Flags::*, ProcessorStatusWord};
pub use ram::{Address, Byte, Ram, Word, WordExt};
pub use register::Registers;
// Word, Byte re-exported from pdp11_common via ram module

mod bootrom;
mod impls;
pub mod insns;
mod psw;
mod ram;
mod register;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Cpu {
    halt: bool,
    registers: Registers,
    psw: ProcessorStatusWord,
    ram: Ram,
    rk: devices::Rk,
    console: devices::Console,
    kw11: devices::Kw11,
    mmio: devices::MmioSpace,
    /// Temporary storage for memory-mapped I/O register reads
    /// This allows returning references to RK11 registers
    io_temp: Word,
    /// Temporary storage for byte I/O reads
    io_temp_byte: Byte,
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
        let rk = devices::Rk::with_image(rk)?;
        let console = devices::Console::new();
        let kw11 = devices::Kw11::new();
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
            kw11,
            mmio: devices::MmioSpace::new(),
            io_temp: Word::zero(),
            io_temp_byte: Byte::zero(),
        };

        Ok(core)
    }

    /// Create a CPU with an empty RK disk for testing purposes
    /// This avoids the need for temporary files in tests
    #[cfg(test)]
    pub fn for_testing() -> Self {
        let rk = devices::Rk::empty();
        let console = devices::Console::new();
        let kw11 = devices::Kw11::new();
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
            kw11,
            mmio: devices::MmioSpace::new(),
            io_temp: Word::zero(),
            io_temp_byte: Byte::zero(),
        }
    }

    pub fn poweron(&mut self) {
        self.reset();
        while !self.halt {
            let opcode = self.next_opcode();
            self.execute(opcode);
            // Check if any RK command was triggered
            self.rk.execute_pending_command(&mut self.ram);
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halt
    }

    pub fn pc_value(&self) -> Word {
        self.registers[PC]
    }

    /// Inject a character into the console input buffer (for testing)
    /// Input a character to the console
    pub fn console_input(&mut self, ch: u8) {
        self.console.input_char(ch);
        // Sync to RAM - but DON'T read RBUF as that clears the DONE bit!
        // Just write the internal console state directly
        self.ram.write_direct(
            devices::console::RCSR,
            self.console.read_register(devices::console::RCSR),
        );
        // For RBUF, we need to access it without triggering the read side-effect
        // So we'll write the character value directly
        self.ram
            .write_direct(devices::console::RBUF, Word::from(u16::from(ch)));
    }

    /// Check for console input and process if available
    pub fn check_console_input(&mut self) {
        self.console.check_input();
        // Sync console state to RAM after checking for input
        self.ram.write_direct(
            devices::console::RCSR,
            self.console.read_register(devices::console::RCSR),
        );
        self.ram.write_direct(
            devices::console::RBUF,
            self.console.read_register(devices::console::RBUF),
        );
    }

    pub fn step(&mut self) {
        let opcode = self.next_opcode();
        self.execute(opcode);
        // Check if any RK command was triggered
        self.rk.execute_pending_command(&mut self.ram);
    }

    fn next_opcode(&mut self) -> Word {
        self.read_word(Operand::pc())
    }

    fn execute(&mut self, opcode: Word) {
        use Instruction::*;
        let instruction = Instruction::from(opcode);
        tracing::trace!("Executing {opcode:#08o}\t{instruction}");

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
            Bhi(offset) => self.bhi(offset),
            Blos(offset) => self.blos(offset),
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
            Rti => self.rti(),
            Xor(register, dst) => self.xor(register, dst),
            Mul(src, register) => self.mul(src, register),
            Div(src, register) => self.div(src, register),
            Ash(src, register) => self.ash(src, register),
            Ashc(src, register) => self.ashc(src, register),
            Sob(register, offset) => self.sob(register, offset),
            Iot => self.iot(),
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
            Invalid(opcode) => tracing::warn!("Opcode {opcode:#08o} is not supported yet"),
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

    pub fn reset(&mut self) {
        self.halt = false;
        self.registers.reset();
        self.psw.reset();
        self.ram.reset();
        self.bootrom();
    }

    fn clr(&mut self, dst: Operand) {
        self.write_word(dst, Word::ZERO);
        self.psw[Z] = true;
        self.psw[N] = false;
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn asl(&mut self, operand: Operand) {
        let result = self.read_word(operand) << 1;
        self.write_word(operand, result);
    }

    /// JMP instruction: transfer control to the effective address
    fn jmp(&mut self, src: Operand) {
        use RegisterAddressingMode::*;

        // For most modes, JMP uses effective_address
        // But for Autoincrement/Autodecrement, we need to read the value from memory
        let target = match src.mode {
            Autoincrement | Autodecrement | AutoincrementDeferred | AutodecrementDeferred => {
                // For these modes, read the target address from memory
                self.read_word(src)
            }
            _ => {
                // For other modes, use the effective address directly
                self.effective_address(src)
            }
        };

        self.registers[PC] = target;
    }

    fn swab(&mut self, dst: Operand) {
        let mut word = self.read_word(dst);
        word.swab();
        self.write_word(dst, word);
        self.psw[Z] = word.is_zero();
        self.psw[N] = word.is_negative();
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn tst(&mut self, src: Operand) {
        let tst = self.read_word(src);
        self.psw[Z] = tst.is_zero();
        self.psw[N] = tst.is_negative();
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn mov(&mut self, src: Operand, dst: Operand) {
        let word = self.read_word(src);
        self.write_word(dst, word);
        self.psw[N] = word.is_negative();
        self.psw[Z] = word.is_zero();
        self.psw[V] = false;
    }

    fn cmp(&mut self, src: Operand, dst: Operand) {
        let src = self.read_word(src);
        let dst = self.read_word(dst);
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
        let src = self.read_word(src);
        let dst = self.read_word(dst);
        let bit = src & dst;
        self.psw[Z] = bit.is_zero();
        self.psw[N] = bit.is_negative();
        self.psw[V] = false;
    }

    fn bic(&mut self, src: Operand, dst: Operand) {
        // BIC: Bit Clear - dst = dst & ~src
        let src = self.read_word(src);
        let dst_val = self.read_word(dst);
        let result = dst_val & !src;
        self.write_word(dst, result);
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
    }

    fn bis(&mut self, src: Operand, dst: Operand) {
        // BIS: Bit Set - dst = dst | src
        let src = self.read_word(src);
        let dst_val = self.read_word(dst);
        let result = dst_val | src;
        self.write_word(dst, result);
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
    }

    fn add(&mut self, src: Operand, dst: Operand) {
        let src_val = self.read_word(src);
        let dst_val = self.read_word(dst);

        // Perform addition with overflow detection
        let src_u16 = src_val.as_u16();
        let dst_u16 = dst_val.as_u16();
        let (result_u16, carry) = dst_u16.overflowing_add(src_u16);
        let result = Word::from(result_u16);

        // Store result
        self.write_word(dst, result);

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
        let src_val = self.read_word(src);
        let dst_val = self.read_word(dst);

        // Perform subtraction with underflow detection
        let src_u16 = src_val.as_u16();
        let dst_u16 = dst_val.as_u16();
        let (result_u16, borrow) = dst_u16.overflowing_sub(src_u16);
        let result = Word::from(result_u16);

        // Store result
        self.write_word(dst, result);

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
        let offset_words = i16::from(offset.0) * 2;
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

    fn bhi(&mut self, offset: Offset) {
        // Branch if Higher (unsigned >: C=0 AND Z=0)
        if !self.psw[C] && !self.psw[Z] {
            self.branch(offset);
        }
    }

    fn blos(&mut self, offset: Offset) {
        // Branch if Lower or Same (unsigned <=: C=1 OR Z=1)
        if self.psw[C] || self.psw[Z] {
            self.branch(offset);
        }
    }

    fn tstb(&mut self, src: Operand) {
        let tstb = self.read_byte(src);
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
                let offset = self.read_word(Operand::pc());
                self.registers[dst.register] + offset
            }
            IndexDeferred => {
                // JSR @X(Rn) means jump to address pointed to by (Rn + X)
                let offset = self.read_word(Operand::pc());
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

    fn rti(&mut self) {
        // RTI: Return from Interrupt
        // Pop PSW from stack
        let sp_addr = self.registers[SP].address::<Word>();
        let psw_word = self.ram[sp_addr];
        self.registers[SP] += 2u16;
        self.psw.from_word(psw_word);

        // Pop PC from stack
        let sp_addr = self.registers[SP].address::<Word>();
        self.registers[PC] = self.ram[sp_addr];
        self.registers[SP] += 2u16;
    }

    fn xor(&mut self, register: Register, dst: Operand) {
        // XOR: Exclusive OR
        let src_val = self.registers[register];
        let dst_val = self.read_word(dst);
        let result = src_val ^ dst_val;

        self.write_word(dst, result);

        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = false;
        // C is not affected
    }

    fn div(&mut self, src: Operand, register: Register) {
        // DIV: Divide
        // Dividend is R|R+1 (32-bit), Divisor is src (16-bit)
        // Quotient -> R, Remainder -> R+1

        let divisor = self.read_word(src).as_i32();

        // Check for divide by zero
        if divisor == 0 {
            self.psw[V] = true; // Set overflow on divide by zero
            self.psw[C] = false;
            return;
        }

        // Build 32-bit dividend from register pair
        let reg_next = Register::from(((register as u8 + 1) & 0o7) as u16);

        let high = self.registers[register].as_i32();
        let low = self.registers[reg_next].as_u16() as i32;
        let dividend = (high << 16) | low;

        let quotient = dividend / divisor;
        let remainder = dividend % divisor;

        // Check for overflow (quotient doesn't fit in 16 bits)
        if !(-32768..=32767).contains(&quotient) {
            self.psw[V] = true;
            self.psw[C] = false;
            return;
        }

        // Store results
        self.registers[register] = Word::from_i32(quotient);
        self.registers[reg_next] = Word::from_i32(remainder);

        // Set flags
        self.psw[N] = (quotient as i16) < 0;
        self.psw[Z] = quotient == 0;
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn mul(&mut self, src: Operand, register: Register) {
        // MUL: Multiply
        // Multiply register by src (both 16-bit signed)
        // Result is 32-bit in register pair R|R+1

        let multiplicand = self.registers[register].as_i32();
        let multiplier = self.read_word(src).as_i32();
        let product = multiplicand.wrapping_mul(multiplier);

        // Store in register pair
        let reg_next = Register::from(((register as u8 + 1) & 0o7) as u16);
        self.registers[register] = Word::from_i32(product >> 16);
        self.registers[reg_next] = Word::from_i32(product);

        // Set flags
        self.psw[N] = product < 0;
        self.psw[Z] = product == 0;
        self.psw[V] = false;
        // C is set if the high-order word is non-zero (result doesn't fit in 16 bits)
        self.psw[C] = (product >> 16) != 0 && (product >> 16) != -1;
    }

    fn ash(&mut self, src: Operand, register: Register) {
        // ASH: Arithmetic Shift
        // Shift register left (positive count) or right (negative count)
        // Count is from bits 0-5 of src (6-bit signed, -32 to +31)

        let shift_count = (self.read_word(src).as_u16() & 0o77) as i8 as i32;
        let value = self.registers[register].as_u16() as i16 as i32;

        let result = if shift_count > 0 {
            // Left shift
            let shift = shift_count.min(31);
            value << shift
        } else if shift_count < 0 {
            // Right shift (arithmetic - sign extends)
            let shift = (-shift_count).min(31);
            value >> shift
        } else {
            value
        };

        self.registers[register] = Word::from(result as i16 as u16);

        // Set flags
        self.psw[N] = (result as i16) < 0;
        self.psw[Z] = (result as i16) == 0;
        // V is set if sign changed during shift
        self.psw[V] = ((value as i16) < 0) != ((result as i16) < 0);
        // C is set if last bit shifted out was 1
        self.psw[C] = if shift_count > 0 {
            // Left shift - check bit that was shifted out
            let shift = shift_count.min(16);
            if shift > 0 {
                let bit_pos = 16 - shift;
                (value >> bit_pos) & 1 != 0
            } else {
                false
            }
        } else if shift_count < 0 {
            // Right shift - check bit that was shifted out
            let shift = (-shift_count).min(16);
            if shift > 0 {
                (value >> (shift - 1)) & 1 != 0
            } else {
                false
            }
        } else {
            false
        };
    }

    fn ashc(&mut self, src: Operand, register: Register) {
        // ASHC: Arithmetic Shift Combined
        // Like ASH but shifts 32-bit register pair R|R+1

        let shift_count = (self.read_word(src).as_u16() & 0o77) as i8 as i64;

        // Build 32-bit value from register pair
        let reg_next = Register::from(((register as u8 + 1) & 0o7) as u16);
        let high = self.registers[register].as_u16() as i16 as i32;
        let low = self.registers[reg_next].as_u16() as i32;
        let value = ((high as i64) << 16) | (low as i64 & 0xFFFF);

        let result = if shift_count > 0 {
            // Left shift
            let shift = shift_count.min(31);
            value << shift
        } else if shift_count < 0 {
            // Right shift (arithmetic - sign extends)
            let shift = (-shift_count).min(31);
            value >> shift
        } else {
            value
        };

        // Store back to register pair
        self.registers[register] = Word::from((result >> 16) as i16 as u16);
        self.registers[reg_next] = Word::from(result as i16 as u16);

        // Set flags
        self.psw[N] = result < 0;
        self.psw[Z] = result == 0;
        // V is set if sign changed during shift
        self.psw[V] = (value < 0) != (result < 0);
        // C is set if last bit shifted out was 1
        self.psw[C] = if shift_count > 0 {
            let shift = shift_count.min(32);
            if shift > 0 {
                let bit_pos = 32 - shift;
                (value >> bit_pos) & 1 != 0
            } else {
                false
            }
        } else if shift_count < 0 {
            let shift = (-shift_count).min(32);
            if shift > 0 {
                (value >> (shift - 1)) & 1 != 0
            } else {
                false
            }
        } else {
            false
        };
    }

    fn sob(&mut self, register: Register, offset: Offset) {
        // SOB: Subtract One and Branch
        // Decrement register, branch backward if not zero
        // Offset is in words (multiply by 2 for byte offset)

        self.registers[register] -= Word::ONE;

        if !self.registers[register].is_zero() {
            // Branch backward by offset words
            let byte_offset = (offset.0 as i16) * 2;
            let current_pc = self.registers[PC].as_u16() as i16;
            self.registers[PC] = Word::from((current_pc - byte_offset) as u16);
        }
    }

    fn iot(&mut self) {
        // IOT: I/O Trap
        // Trap to vector 020 (octal)
        tracing::debug!("IOT instruction not fully implemented - would trap to vector 020");
        // For now, just log it. Full implementation would:
        // 1. Push PSW to stack
        // 2. Push PC to stack
        // 3. Load new PC from vector 020
        // 4. Load new PSW from vector 022
    }

    fn com(&mut self, dst: Operand) {
        // COM: Complement (one's complement)
        let result = !self.read_word(dst);
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = false;
        self.psw[C] = true; // Always set
    }

    fn inc(&mut self, dst: Operand) {
        // INC: Increment
        let value = self.read_word(dst);
        let result = value + Word::ONE;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Word::MAX_POSITIVE; // Overflow from max positive
    }

    fn dec(&mut self, dst: Operand) {
        // DEC: Decrement
        let value = self.read_word(dst);
        let result = value - Word::ONE;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Word::MIN_NEGATIVE; // Overflow from min negative
    }

    fn neg(&mut self, dst: Operand) {
        // NEG: Negate (two's complement)
        let value = self.read_word(dst);
        let result = -value;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Word::MIN_NEGATIVE; // Overflow from min negative
        self.psw[C] = !result.is_zero(); // Clear if result is zero
    }

    fn adc(&mut self, dst: Operand) {
        // ADC: Add Carry
        let value = self.read_word(dst);
        let carry = if self.psw[C] { Word::ONE } else { Word::ZERO };
        let result = value + carry;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=077777 (max positive)
        self.psw[V] = carry == Word::ONE && value == Word::MAX_POSITIVE;
        // Carry if value=177777 and carry=1
        self.psw[C] = carry == Word::ONE && value == Word::MAX_UNSIGNED;
    }

    fn sbc(&mut self, dst: Operand) {
        // SBC: Subtract Carry
        let value = self.read_word(dst);
        let carry = if self.psw[C] { Word::ONE } else { Word::ZERO };
        let result = value - carry;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=100000 (min negative)
        self.psw[V] = carry == Word::ONE && value == Word::MIN_NEGATIVE;
        // Carry set if result borrows (value was 0 and carry was 1)
        self.psw[C] = carry == Word::ONE && value.is_zero();
    }

    fn ror(&mut self, dst: Operand) {
        // ROR: Rotate Right through carry
        let value = self.read_word(dst);
        let old_carry = if self.psw[C] { Word::ONE } else { Word::ZERO };
        let new_carry = value.as_u16() & 1;
        let result = (value >> 1) | (old_carry << 15);
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn rol(&mut self, dst: Operand) {
        // ROL: Rotate Left through carry
        let value = self.read_word(dst);
        let old_carry = if self.psw[C] { Word::ONE } else { Word::ZERO };
        let new_carry = (value.as_u16() >> 15) & 1;
        let result = (value << 1) | old_carry;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn asr(&mut self, dst: Operand) {
        // ASR: Arithmetic Shift Right (sign-extend)
        let value = self.read_word(dst);
        let sign_bit = Word::from_u16(value.as_u16() & 0o100000);
        let new_carry = value.as_u16() & 1;
        let result = (value >> 1) | sign_bit;
        self.write_word(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    #[expect(clippy::unused_self)]
    fn nop(&self) {
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
        self.write_byte(dst, Byte::ZERO);
        self.psw[Z] = true;
        self.psw[N] = false;
        self.psw[V] = false;
        self.psw[C] = false;
    }

    fn comb(&mut self, dst: Operand) {
        // COMB: Complement byte (one's complement)
        let result = !self.read_byte(dst);
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = false;
        self.psw[C] = true; // Always set
    }

    fn incb(&mut self, dst: Operand) {
        // INCB: Increment byte
        let value = self.read_byte(dst);
        let result = value + Byte::ONE;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Byte::MAX_POSITIVE; // Overflow from max positive
    }

    fn decb(&mut self, dst: Operand) {
        // DECB: Decrement byte
        let value = self.read_byte(dst);
        let result = value - Byte::ONE;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Byte::MIN_NEGATIVE; // Overflow from min negative
    }

    fn negb(&mut self, dst: Operand) {
        // NEGB: Negate byte (two's complement)
        let value = self.read_byte(dst);
        let result = -value;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[V] = value == Byte::MIN_NEGATIVE; // Overflow from min negative
        self.psw[C] = !result.is_zero(); // Clear if result is zero
    }

    fn adcb(&mut self, dst: Operand) {
        // ADCB: Add Carry to byte
        let value = self.read_byte(dst);
        let carry = if self.psw[C] { Byte::ONE } else { Byte::ZERO };
        let result = value + carry;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=0177 (max positive)
        self.psw[V] = carry == Byte::ONE && value == Byte::MAX_POSITIVE;
        // Carry if value=0377 and carry=1
        self.psw[C] = carry == Byte::ONE && value == Byte::MAX_UNSIGNED;
    }

    fn sbcb(&mut self, dst: Operand) {
        // SBCB: Subtract Carry from byte
        let value = self.read_byte(dst);
        let carry = if self.psw[C] { Byte::ONE } else { Byte::ZERO };
        let result = value - carry;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        // Overflow if carry=1, value=0200 (min negative)
        self.psw[V] = carry == Byte::ONE && value == Byte::MIN_NEGATIVE;
        // Carry set if result borrows (value was 0 and carry was 1)
        self.psw[C] = carry == Byte::ONE && value.is_zero();
    }

    fn rorb(&mut self, dst: Operand) {
        // RORB: Rotate Right byte through carry
        let value = self.read_byte(dst);
        let old_carry = if self.psw[C] { Byte::ONE } else { Byte::ZERO };
        let new_carry = value.as_u8() & 1;
        let result = (value >> 1) | (old_carry << 7);
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn rolb(&mut self, dst: Operand) {
        // ROLB: Rotate Left byte through carry
        let value = self.read_byte(dst);
        let old_carry = if self.psw[C] { Byte::ONE } else { Byte::ZERO };
        let new_carry = (value.as_u8() >> 7) & 1;
        let result = (value << 1) | old_carry;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn asrb(&mut self, dst: Operand) {
        // ASRB: Arithmetic Shift Right byte (sign-extend)
        let value = self.read_byte(dst);
        let sign_bit = Byte::from(value.as_u8() & 0o200);
        let new_carry = value.as_u8() & 1;
        let result = (value >> 1) | sign_bit;
        self.write_byte(dst, result);
        self.psw[N] = result.is_negative();
        self.psw[Z] = result.is_zero();
        self.psw[C] = new_carry != 0;
        self.psw[V] = self.psw[N] != self.psw[C]; // N xor C
    }

    fn movb(&mut self, src: Operand, dst: Operand) {
        let byte = self.read_byte(src);
        self.write_byte(dst, byte);
        self.psw[N] = byte.is_negative();
        self.psw[Z] = byte.is_zero();
        self.psw[V] = false;
    }

    fn cmpb(&mut self, src: Operand, dst: Operand) {
        let src = self.read_byte(src);
        let dst = self.read_byte(dst);
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
        let src = self.read_byte(src);
        let dst = self.read_byte(dst);
        let bit = src & dst;
        self.psw[Z] = bit.is_zero();
        self.psw[N] = bit.is_negative();
        self.psw[V] = false;
    }

    fn bicb(&mut self, src: Operand, dst: Operand) {
        // BICB: Bit Clear Byte - dst = dst & ~src
        let src = self.read_byte(src);
        let dst_val = self.read_byte(dst);
        let result = dst_val & !src;
        self.write_byte(dst, result);
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
    }

    fn bisb(&mut self, src: Operand, dst: Operand) {
        // BISB: Bit Set Byte - dst = dst | src
        let src = self.read_byte(src);
        let dst_val = self.read_byte(dst);
        let result = dst_val | src;
        self.write_byte(dst, result);
        self.psw[Z] = result.is_zero();
        self.psw[N] = result.is_negative();
        self.psw[V] = false;
        // C is unaffected
    }

    // Interrupt handling

    /// Trigger an interrupt with the given vector address and priority level
    /// Vector address points to PC/PSW pair in low memory
    #[cfg(test)]
    pub(crate) fn interrupt(&mut self, vector: u16, priority: u8) {
        // Only process interrupt if its priority is higher than current IPL
        if priority <= self.psw.priority() {
            return;
        }

        // Push current PC to stack
        self.registers[SP] -= 2u16;
        let sp_addr = self.registers[SP].address::<Word>();
        self.ram.write_direct(sp_addr, self.registers[PC]);

        // Push current PSW to stack
        self.registers[SP] -= 2u16;
        let sp_addr = self.registers[SP].address::<Word>();
        self.ram.write_direct(sp_addr, self.psw.as_word());

        // Load new PC from vector
        let vec_addr = Address::<Word>::from_u16(vector);
        self.registers[PC] = self.ram[vec_addr];

        // Load new PSW from vector + 2 (includes new priority level)
        let psw_addr = Address::<Word>::from_u16(vector + 2);
        self.psw.from_word(self.ram[psw_addr]);
    }

    /// Check for pending peripheral interrupts
    /// Returns (vector, priority) if interrupt should be serviced
    #[cfg(test)]
    pub(crate) fn check_interrupts(&mut self) -> Option<(u16, u8)> {
        // Check interrupts in priority order (highest first)

        // Priority 6: KW11-L line clock (vector 0o100)
        if self.kw11.interrupt_pending() {
            return Some((devices::kw11::KW11_VECTOR, devices::kw11::KW11_PRIORITY));
        }

        // Priority 5: RK11 disk (vector 0o220)
        if self.rk.interrupt_pending() {
            return Some((devices::rk::RK11_VECTOR, devices::rk::RK11_PRIORITY));
        }

        // Priority 4: Console receiver (vector 0o060)
        if self.console.rx_interrupt_pending() {
            return Some((
                devices::console::CONSOLE_RX_VECTOR,
                devices::console::CONSOLE_PRIORITY,
            ));
        }

        // Priority 4: Console transmitter (vector 0o064)
        if self.console.tx_interrupt_pending() {
            return Some((
                devices::console::CONSOLE_TX_VECTOR,
                devices::console::CONSOLE_PRIORITY,
            ));
        }

        None
    }

    /// Tick the KW11-L line clock
    /// Should be called periodically (e.g., every ~16.67ms for 60Hz)
    /// Returns true if an interrupt was generated
    #[allow(dead_code)]
    fn tick_kw11(&mut self) -> bool {
        self.kw11.tick()
    }

    /// Clear KW11-L interrupt (called after servicing)
    #[cfg(test)]
    #[expect(dead_code)]
    fn clear_kw11_interrupt(&mut self) {
        self.kw11.clear_interrupt();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Operand {
    mode: RegisterAddressingMode,
    register: Register,
}

impl Operand {
    pub(super) fn from_0_5(opcode: u16) -> Self {
        let mode = RegisterAddressingMode::from((opcode & 0o000070) >> 3);
        let register = Register::from(opcode & 0o000007);
        Self { mode, register }
    }

    pub(super) fn from_6_11(opcode: u16) -> Self {
        let mode = RegisterAddressingMode::from((opcode & 0o007000) >> 9);
        let register = Register::from((opcode & 0o000700) >> 6);
        Self { mode, register }
    }

    pub(super) fn pc() -> Self {
        Self {
            mode: RegisterAddressingMode::Autoincrement,
            register: PC,
        }
    }

    // Helper constructors for tests
    #[cfg(test)]
    pub fn reg(register: Register) -> Self {
        Self {
            mode: RegisterAddressingMode::Register,
            register,
        }
    }

    #[cfg(test)]
    pub fn register_deferred(register: Register) -> Self {
        Self {
            mode: RegisterAddressingMode::RegisterDeferred,
            register,
        }
    }

    #[cfg(test)]
    pub fn autoincrement(register: Register) -> Self {
        Self {
            mode: RegisterAddressingMode::Autoincrement,
            register,
        }
    }

    #[cfg(test)]
    pub fn autodecrement(register: Register) -> Self {
        Self {
            mode: RegisterAddressingMode::Autodecrement,
            register,
        }
    }

    #[cfg(test)]
    pub fn index(register: Register) -> Self {
        Self {
            mode: RegisterAddressingMode::Index,
            register,
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

#[derive(Debug, Clone, Copy)]
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
    fn is_zero(&self) -> bool;
    fn is_negative(&self) -> bool;
}

#[cfg(test)]
mod tests;
