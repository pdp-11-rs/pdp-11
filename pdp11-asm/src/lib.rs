//! MACRO-11 Compatible Assembler for PDP-11
//!
//! This assembler provides MACRO-11 compatible assembly language support
//! for the PDP-11 architecture.

pub mod addressing;
pub mod encoder;
pub mod error;
pub mod expr;
pub mod lexer;
pub mod parser;
pub mod symbol;

pub use addressing::AddressingMode;
pub use encoder::encode_instruction;
pub use error::{AsmError, Result};
pub use lexer::Lexer;
pub use parser::Parser;
pub use symbol::SymbolTable;

/// An assembled instruction or directive
#[derive(Debug, Clone)]
pub enum Statement {
    /// Machine instruction
    Instruction(Instruction),
    /// Assembler directive
    Directive(Directive),
    /// Label definition
    Label(String),
}

/// A PDP-11 instruction ready to be encoded
#[derive(Debug, Clone)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

/// Instruction mnemonics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    // Double operand
    Mov,
    Movb,
    Cmp,
    Cmpb,
    Bit,
    Bitb,
    Bic,
    Bicb,
    Bis,
    Bisb,
    Add,
    Sub,

    // Single operand
    Clr,
    Clrb,
    Com,
    Comb,
    Inc,
    Incb,
    Dec,
    Decb,
    Neg,
    Negb,
    Adc,
    Adcb,
    Sbc,
    Sbcb,
    Tst,
    Tstb,
    Ror,
    Rorb,
    Rol,
    Rolb,
    Asr,
    Asrb,
    Asl,
    Aslb,
    Jmp,
    Swab,

    // Branch
    Br,
    Bne,
    Beq,
    Bge,
    Blt,
    Bgt,
    Ble,
    Bpl,
    Bmi,
    Bhi,
    Blos,
    Bvc,
    Bvs,
    Bcc,
    Bcs,

    // Subroutine
    Jsr,
    Rts,

    // EIS
    Mul,
    Div,
    Ash,
    Ashc,
    Xor,
    Sob,

    // Control
    Halt,
    Wait,
    Rti,
    Iot,
    Reset,
    Nop,

    // Condition codes
    Clc,
    Sec,
    Clv,
    Sev,
    Clz,
    Sez,
    Cln,
    Sen,
    Ccc,
    Scc,
}

/// Operand in an instruction
#[derive(Debug, Clone)]
pub struct Operand {
    pub mode: AddressingMode,
    pub value: Option<Expr>,
}

/// Expression in an operand or directive
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Symbol(String),
    CurrentLocation,
    Binary {
        op: BinaryOp,
        left: Box<Self>,
        right: Box<Self>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Self>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Complement,
}

/// Assembler directives
#[derive(Debug, Clone)]
pub enum Directive {
    /// .WORD value, value, ...
    Word(Vec<Expr>),
    /// .BYTE value, value, ...
    Byte(Vec<Expr>),
    /// .ASCII "string"
    Ascii(String),
    /// .ASCIZ "string" (null-terminated)
    Asciz(String),
    /// .BLKW count
    BlkW(Expr),
    /// .BLKB count
    BlkB(Expr),
    /// .EVEN (align to word boundary)
    Even,
    /// .ODD (align to odd address)
    Odd,
    /// . = expression (set location counter)
    SetLocation(Expr),
    /// .TITLE "text"
    Title(String),
    /// .SBTTL "text"
    Sbttl(String),
    /// .END [start_address]
    End(Option<Expr>),
}
