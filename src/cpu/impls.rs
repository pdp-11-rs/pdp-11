use super::*;

impl Cpu {
    pub(super) fn word(&mut self, operand: Operand) -> &Word {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => &self.registers[register],
            RegisterDeferred => {
                let address = self.registers[register].address();
                self.ram.word(address)
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Word>(register).address();
                self.ram.word(address)
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.word(address)
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Word>(register).address();
                self.ram.word(address)
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.word(address)
            }
            Index => todo!("src word index"),
            IndexDeferred => todo!("src word index deferred"),
        }
    }

    pub(super) fn word_mut(&mut self, operand: Operand) -> &mut Word {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => &mut self.registers[register],
            RegisterDeferred => {
                let address = self.registers[register].address();
                self.ram.word_mut(address)
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Word>(register).address();
                self.ram.word_mut(address)
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.word_mut(address)
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Word>(register).address();
                self.ram.word_mut(address)
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.word_mut(address)
            }
            Index => todo!("src word index"),
            IndexDeferred => todo!("src word index deferred"),
        }
    }

    pub(super) fn byte(&mut self, operand: Operand) -> &Byte {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => self.registers[register].byte(0),
            RegisterDeferred => {
                let address = self.registers[register].address();
                self.ram.byte(address)
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Byte>(register).address();
                self.ram.byte(address)
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.byte(address)
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Byte>(register).address();
                self.ram.byte(address)
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.byte(address)
            }
            Index => todo!("src word index"),
            IndexDeferred => todo!("src word index deferred"),
        }
    }

    pub(super) fn byte_mut(&mut self, operand: Operand) -> &mut Byte {
        use RegisterAddressingMode::*;

        let Operand { mode, register } = operand;

        match mode {
            Register => self.registers[register].byte_mut(0),
            RegisterDeferred => {
                let address = self.registers[register].address();
                self.ram.byte_mut(address)
            }
            Autoincrement => {
                let address = self.registers.get_inc::<Byte>(register).address();
                self.ram.byte_mut(address)
            }
            AutoincrementDeferred => {
                let address = self.registers.get_inc::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.byte_mut(address)
            }
            Autodecrement => {
                let address = self.registers.dec_get::<Byte>(register).address();
                self.ram.byte_mut(address)
            }
            AutodecrementDeferred => {
                let address = self.registers.dec_get::<Word>(register).address();
                let address = self.ram.word(address).address();
                self.ram.byte_mut(address)
            }
            Index => todo!("src word index"),
            IndexDeferred => todo!("src word index deferred"),
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
}
