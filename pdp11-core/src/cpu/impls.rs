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
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
                } else {
                    &self.ram[address]
                }
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Byte>(register).address::<Byte>();
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
                } else {
                    &self.ram[address]
                }
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
                } else {
                    &self.ram[address]
                }
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Byte>(register).address::<Byte>();
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
                } else {
                    &self.ram[address]
                }
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
                } else {
                    &self.ram[address]
                }
            }
            Index => {
                // Get index offset from next word in instruction stream
                let offset = *self.word(Operand::pc());
                // Add offset to register value to get final address
                let base = self.registers[register];
                let address = (base + offset).address::<Byte>();
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
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
                let address = self.ram[address].address::<Byte>();
                // Check for MMIO
                if self.mmio.is_io_space_byte(address) {
                    self.io_temp_byte = self.read_mmio_byte(address);
                    &self.io_temp_byte
                } else {
                    &self.ram[address]
                }
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

    /// Write a word to memory with MMIO routing
    pub(super) fn write_word(&mut self, operand: Operand, value: Word) {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => {
                self.registers[register] = value;
            }
            RegisterDeferred => {
                let address = self.registers[register].address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            Index => {
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            IndexDeferred => {
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                let address = self.ram[address].address::<Word>();
                if self.mmio.is_io_space_word(address) {
                    self.write_mmio_word(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
        }
    }

    /// Write a byte to memory with MMIO routing
    pub(super) fn write_byte(&mut self, operand: Operand, value: Byte) {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => {
                *self.registers[register].byte_mut(0) = value;
            }
            RegisterDeferred => {
                let address = self.registers[register].address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Byte>(register).address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Byte>(register).address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            Index => {
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                let address = (base + offset).address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
            }
            IndexDeferred => {
                let offset = *self.word(Operand::pc());
                let base = self.registers[register];
                let address = (base + offset).address::<Word>();
                let address = self.ram[address].address::<Byte>();
                if self.mmio.is_io_space_byte(address) {
                    self.write_mmio_byte(address, value);
                } else {
                    self.ram[address] = value;
                }
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
        use devices::mmio::MmioDevice;
        self.console.handles_word_address(address)
    }

    /// Check if an address is in RK11 I/O space
    #[allow(dead_code)]
    fn is_rk_io(&self, address: Address<Word>) -> bool {
        use devices::mmio::MmioDevice;
        self.rk.handles_word_address(address)
    }

    /// Check if an address is in any MMIO space
    #[allow(dead_code)]
    fn is_mmio(&self, address: Address<Word>) -> bool {
        self.mmio.is_io_space_word(address)
    }

    /// Read from MMIO device (word)
    #[allow(dead_code)]
    fn read_mmio_word(&mut self, address: Address<Word>) -> Word {
        use devices::mmio::MmioDevice;
        if self.console.handles_word_address(address) {
            self.console.read_word(address)
        } else if self.rk.handles_word_address(address) {
            self.rk.read_word(address)
        } else if self.kw11.handles_word_address(address) {
            self.kw11.read_word(address)
        } else {
            Word::zero() // Unimplemented I/O address
        }
    }

    /// Write to MMIO device (word)
    #[allow(dead_code)]
    fn write_mmio_word(&mut self, address: Address<Word>, value: Word) {
        use devices::mmio::MmioDevice;
        if self.console.handles_word_address(address) {
            self.console.write_word(address, value);
        } else if self.rk.handles_word_address(address) {
            self.rk.write_word(address, value);
            // Check if RK command was triggered
            if address == devices::rk::RKCS {
                self.rk.execute_pending_command(&mut self.ram);
            }
        } else if self.kw11.handles_word_address(address) {
            self.kw11.write_word(address, value);
        }
    }

    /// Read from MMIO device (byte)
    #[allow(dead_code)]
    fn read_mmio_byte(&mut self, address: Address<Byte>) -> Byte {
        use devices::mmio::MmioDevice;
        if self.console.handles_byte_address(address) {
            self.console.read_byte(address)
        } else if self.rk.handles_byte_address(address) {
            self.rk.read_byte(address)
        } else if self.kw11.handles_byte_address(address) {
            self.kw11.read_byte(address)
        } else {
            Byte::zero() // Unimplemented I/O address
        }
    }

    /// Write to MMIO device (byte)
    #[allow(dead_code)]
    fn write_mmio_byte(&mut self, address: Address<Byte>, value: Byte) {
        use devices::mmio::MmioDevice;
        if self.console.handles_byte_address(address) {
            self.console.write_byte(address, value);
        } else if self.rk.handles_byte_address(address) {
            self.rk.write_byte(address, value);
            // Check if RK command was triggered
            let word_addr = Address::<Word>::from_u16(address.word_index() as u16 * 2);
            if word_addr == devices::rk::RKCS {
                self.rk.execute_pending_command(&mut self.ram);
            }
        } else if self.kw11.handles_byte_address(address) {
            self.kw11.write_byte(address, value);
        }
    }

    /// Read from console register (for MMIO)
    pub(super) fn read_console(&mut self, address: Address<Word>) -> Word {
        use devices::mmio::MmioDevice;
        self.console.read_word(address)
    }

    /// Write to console register (for MMIO)
    pub(super) fn write_console(&mut self, address: Address<Word>, value: Word) {
        use devices::mmio::MmioDevice;
        self.console.write_word(address, value);
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
