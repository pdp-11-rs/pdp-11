use super::*;

impl Cpu {
    pub(super) fn load<M>(&mut self, src: Source) -> M
    where
        M: MemoryAcceess,
    {
        use RegisterAddressingMode::*;

        let Source { mode, register } = src;

        match mode {
            Register => self.registers[register].into(),
            RegisterDeferred => self.load_indirect(register),
            Autoincrement => {
                let out = self.load_indirect(register);
                self.registers.inc::<M>(register);
                out
            }
            AutoincrementDeferred => {
                let out = self.load_indirect2(register);
                self.registers.inc::<Word>(register);
                out
            }
            Autodecrement => {
                self.registers.dec::<M>(register);
                self.load_indirect(register)
            }
            AutodecrementDeferred => {
                self.registers.dec::<Word>(register);
                self.load_indirect2(register)
            }
            Index => todo!("load index"),
            IndexDeferred => todo!("load index deferred"),
        }
    }

    pub(super) fn store<M>(&mut self, dst: Destination, data: M)
    where
        M: MemoryAcceess,
        // Word: From<M>,
    {
        use RegisterAddressingMode::*;

        let Destination { mode, register } = dst;

        match mode {
            Register => {
                self.registers[register] = data.into();
            }
            RegisterDeferred => {
                self.store_indirect(register, data);
            }
            Autoincrement => {
                self.store_indirect(register, data);
                self.registers.inc::<M>(register);
            }
            AutoincrementDeferred => {
                self.store_indirect2(register, data);
                self.registers.inc::<Word>(register);
            }
            Autodecrement => {
                self.registers.dec::<M>(register);
                self.store_indirect(register, data);
            }
            AutodecrementDeferred => {
                self.registers.dec::<Word>(register);
                self.store_indirect2(register, data);
            }
            Index => todo!(),
            IndexDeferred => todo!(),
        };
    }

    fn load_indirect<M>(&self, register: Register) -> M
    where
        M: MemoryAcceess,
    {
        let address = self.registers[register].address();
        self.ram.load(address)
    }

    fn load_indirect2<M>(&self, register: Register) -> M
    where
        M: MemoryAcceess,
    {
        let address = self.registers[register].address();
        let address = self.ram.load::<Word>(address).address();
        self.ram.load(address)
    }

    fn store_indirect<M>(&mut self, register: Register, data: M)
    where
        M: MemoryAcceess,
    {
        let address = self.registers[register].address();
        self.ram.store(address, data);
    }

    fn store_indirect2<M>(&mut self, register: Register, data: M)
    where
        M: MemoryAcceess,
    {
        let address = self.registers[register].address();
        let address = self.ram.load::<Word>(address).address();
        self.ram.store(address, data);
    }
}
