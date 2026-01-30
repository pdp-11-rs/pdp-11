use super::*;

impl Cpu {
    pub(super) fn word(&mut self, operand: Operand) -> &Word {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => &self.registers[register],
            RegisterDeferred => {
                let address = self.registers[register].address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
            Index => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value to get final address
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
            IndexDeferred => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                // Dereference to get final address
                let address = self.ram[address].address::<Word>();
                // Check for console MMIO
                if self.is_console_io(address) {
                    self.io_temp = self.read_console(address);
                    &self.io_temp
                } else {
                    &self.ram[address]
                }
            }
        }
    }

    pub(super) fn word_mut(&mut self, operand: Operand) -> &mut Word {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => &mut self.registers[register],
            RegisterDeferred => {
                let address = self.registers[register].address::<Word>();
                &mut self.ram[address]
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                &mut self.ram[address]
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Word>();
                &mut self.ram[address]
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                &mut self.ram[address]
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Word>();
                &mut self.ram[address]
            }
            Index => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value to get final address
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                &mut self.ram[address]
            }
            IndexDeferred => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                // Dereference to get final address
                let address = self.ram[address].address::<Word>();
                &mut self.ram[address]
            }
        }
    }

    pub(super) fn byte(&mut self, operand: Operand) -> &Byte {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => self.registers[register].byte(0),
            RegisterDeferred => {
                let address = self.registers[register].address::<Byte>();
                &self.ram[address]
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Byte>(register).address::<Byte>();
                &self.ram[address]
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                &self.ram[address]
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Byte>(register).address::<Byte>();
                &self.ram[address]
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                &self.ram[address]
            }
            Index => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value to get final address
                let base = self.registers[register];
                let address = (base + offset).address::<Byte>();
                &self.ram[address]
            }
            IndexDeferred => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                // Dereference to get final address
                let address = self.ram[address].address::<Byte>();
                &self.ram[address]
            }
        }
    }

    pub(super) fn byte_mut(&mut self, operand: Operand) -> &mut Byte {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => self.registers[register].byte_mut(0),
            RegisterDeferred => {
                let address = self.registers[register].address::<Byte>();
                &mut self.ram[address]
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Byte>(register).address::<Byte>();
                &mut self.ram[address]
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                &mut self.ram[address]
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Byte>(register).address::<Byte>();
                &mut self.ram[address]
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                &mut self.ram[address]
            }
            Index => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value to get final address
                let base = self.registers[register];
                let address = (base + offset).address::<Byte>();
                &mut self.ram[address]
            }
            IndexDeferred => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                // Dereference to get final address
                let address = self.ram[address].address::<Byte>();
                &mut self.ram[address]
            }
        }
    }

    /// Get the effective address for an operand (used for instructions like JMP)
    pub(super) fn effective_address(&mut self, operand: Operand) -> Word {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => {
                // Register mode: effective address is the contents of the register
                self.registers[register]
            }
            RegisterDeferred => {
                // Register deferred: effective address is the contents of the register
                self.registers[register]
            }
            Autoincrement => {
                // Autoincrement: effective address is register contents, then increment
                self.registers.get_inc::<Word>(register)
            }
            AutoincrementDeferred => {
                // Autoincrement deferred: get address from register (increment it),
                // then get effective address from memory at that location
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                self.ram[address]
            }
            Autodecrement => {
                // Autodecrement: decrement register first, then use as effective address
                self.registers.dec_get::<Word>(register)
            }
            AutodecrementDeferred => {
                // Autodecrement deferred: decrement register, get address from memory
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                self.ram[address]
            }
            Index => {
                // Index: get offset from next word, add to register value
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                base + offset
            }
            IndexDeferred => {
                // Index deferred: get offset, add to register, then dereference
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                self.ram[address]
            }
        }
    }

    // pub(super) fn load<M>(&mut self, src: Operand) -> M
    // where
    //     M: MemoryAcceess,
    // {
    //     use RegisterAddressingMode::*;

    //     let Operand { mode, register } = src;

    //     match mode {
    //         Register => self.registers[register].into(),
    //         RegisterDeferred => self.load_indirect(register),
    //         Autoincrement => {
    //             let out = self.load_indirect(register);
    //             self.registers.inc::<M>(register);
    //             out
    //         }
    //         AutoincrementDeferred => {
    //             let out = self.load_indirect2(register);
    //             self.registers.inc::<Word>(register);
    //             out
    //         }
    //         Autodecrement => {
    //             self.registers.dec::<M>(register);
    //             self.load_indirect(register)
    //         }
    //         AutodecrementDeferred => {
    //             self.registers.dec::<Word>(register);
    //             self.load_indirect2(register)
    //         }
    //         Index => todo!("load index"),
    //         IndexDeferred => todo!("load index deferred"),
    //     }
    // }

    // pub(super) fn store<M>(&mut self, dst: Operand, data: M)
    // where
    //     M: MemoryAcceess,
    // {
    //     use RegisterAddressingMode::*;

    //     let Operand { mode, register } = dst;

    //     match mode {
    //         Register => {
    //             self.registers[register] = data.into();
    //         }
    //         RegisterDeferred => {
    //             self.store_indirect(register, data);
    //         }
    //         Autoincrement => {
    //             self.store_indirect(register, data);
    //             self.registers.inc::<M>(register);
    //         }
    //         AutoincrementDeferred => {
    //             self.store_indirect2(register, data);
    //             self.registers.inc::<Word>(register);
    //         }
    //         Autodecrement => {
    //             self.registers.dec::<M>(register);
    //             self.store_indirect(register, data);
    //         }
    //         AutodecrementDeferred => {
    //             self.registers.dec::<Word>(register);
    //             self.store_indirect2(register, data);
    //         }
    //         Index => todo!(),
    //         IndexDeferred => todo!(),
    //     };
    // }

    // fn load_indirect<M>(&self, register: Register) -> M
    // where
    //     M: MemoryAcceess,
    // {
    //     let address = self.registers[register].address();
    //     self.ram.load(address)
    // }

    // fn load_indirect2<M>(&self, register: Register) -> M
    // where
    //     M: MemoryAcceess,
    // {
    //     let address = self.registers[register].address();
    //     let address = self.ram.load::<Word>(address).address();
    //     self.ram.load(address)
    // }

    // fn store_indirect<M>(&mut self, register: Register, data: M)
    // where
    //     M: MemoryAcceess,
    // {
    //     let address = self.registers[register].address();
    //     self.ram.store(address, data);
    // }

    // fn store_indirect2<M>(&mut self, register: Register, data: M)
    // where
    //     M: MemoryAcceess,
    // {
    //     let address: ram::Address<Word> = self.registers[register].address();
    //     let address = self.ram.load::<Word>(address).address();
    //     self.ram.store(address, data);
    // }

    /// Check if an address is in console I/O space and handle it
    pub(super) fn is_console_io(&self, address: Address<Word>) -> bool {
        matches!(
            address,
            console::RCSR | console::RBUF | console::XCSR | console::XBUF
        )
    }

    /// Read from console register (for MMIO)
    pub(super) fn read_console(&mut self, address: Address<Word>) -> Word {
        self.console.read_register(address)
    }

    /// Write to console register (for MMIO)
    pub(super) fn write_console(&mut self, address: Address<Word>, value: Word) {
        self.console.write_register(address, value);
    }

    /// Get the memory address for an operand (if applicable)
    /// Returns None for register-direct mode
    pub(super) fn get_operand_address(&mut self, operand: Operand) -> Option<Address<Word>> {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => None,
            RegisterDeferred => Some(self.registers[register].address::<Word>()),
            Autoincrement => {
                let address = self.registers[register].address::<Word>();
                self.registers[register] += 2u16;
                Some(address)
            }
            AutoincrementDeferred => {
                let address = self.registers[register].address::<Word>();
                self.registers[register] += 2u16;
                Some(self.ram[address].address::<Word>())
            }
            Autodecrement => {
                self.registers[register] -= 2u16;
                Some(self.registers[register].address::<Word>())
            }
            AutodecrementDeferred => {
                self.registers[register] -= 2u16;
                let address = self.registers[register].address::<Word>();
                Some(self.ram[address].address::<Word>())
            }
            Index => {
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                Some((base + offset).address::<Word>())
            }
            IndexDeferred => {
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                Some(self.ram[address].address::<Word>())
            }
        }
    }
}
