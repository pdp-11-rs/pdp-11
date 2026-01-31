use pdp11_common::Register;

/// PDP-11 addressing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Register: R0
    Register(Register),

    /// Register Deferred: (R0)
    RegisterDeferred(Register),

    /// Autoincrement: (R0)+
    Autoincrement(Register),

    /// Autoincrement Deferred: @(R0)+
    AutoincrementDeferred(Register),

    /// Autodecrement: -(R0)
    Autodecrement(Register),

    /// Autodecrement Deferred: @-(R0)
    AutodecrementDeferred(Register),

    /// Index: X(R0)
    Index(Register),

    /// Index Deferred: @X(R0)
    IndexDeferred(Register),

    /// Immediate: #value (actually PC autoincrement)
    Immediate,

    /// Absolute: @#value (actually PC autoincrement deferred)
    Absolute,

    /// Relative: addr (actually PC index)
    Relative,

    /// Relative Deferred: @addr (actually PC index deferred)
    RelativeDeferred,
}

impl AddressingMode {
    /// Encode addressing mode to 6-bit field (mode:3, reg:3)
    pub fn encode(&self) -> u16 {
        use AddressingMode::*;
        match self {
            Register(r) => r.to_code(),
            RegisterDeferred(r) => (1 << 3) | r.to_code(),
            Autoincrement(r) => (2 << 3) | r.to_code(),
            AutoincrementDeferred(r) => (3 << 3) | r.to_code(),
            Autodecrement(r) => (4 << 3) | r.to_code(),
            AutodecrementDeferred(r) => (5 << 3) | r.to_code(),
            Index(r) => (6 << 3) | r.to_code(),
            IndexDeferred(r) => (7 << 3) | r.to_code(),
            Immediate => (2 << 3) | 7,        // (PC)+
            Absolute => (3 << 3) | 7,         // @(PC)+
            Relative => (6 << 3) | 7,         // X(PC)
            RelativeDeferred => (7 << 3) | 7, // @X(PC)
        }
    }

    /// Does this mode require an additional word?
    pub fn needs_extra_word(&self) -> bool {
        use AddressingMode::*;
        matches!(
            self,
            Index(_) | IndexDeferred(_) | Immediate | Absolute | Relative | RelativeDeferred
        )
    }
}
