use crate::error::{AsmError, Result};
use crate::lexer::{Lexer, Token};
use crate::{Directive, Instruction, Mnemonic, Operand, Statement};
#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    pub fn parse_line(&mut self) -> Result<Option<Statement>> {
        // Skip empty lines
        while matches!(self.current, Token::Newline) {
            self.advance()?;
        }

        if matches!(self.current, Token::Eof) {
            return Ok(None);
        }

        // Check for label
        if let Token::Identifier(name) = &self.current {
            let next = self.lexer.next_token()?;
            if matches!(next, Token::Colon) {
                let label = name.clone();
                self.current = self.lexer.next_token()?;
                return Ok(Some(Statement::Label(label)));
            }
            // Not a label, put back the token
            self.current = Token::Identifier(name.clone());
        }

        // Check for directive
        if let Token::Identifier(name) = &self.current
            && name.starts_with('.')
        {
            return Ok(Some(Statement::Directive(self.parse_directive()?)));
        }

        // Must be an instruction
        Ok(Some(Statement::Instruction(self.parse_instruction()?)))
    }

    fn parse_instruction(&mut self) -> Result<Instruction> {
        let mnemonic = self.parse_mnemonic()?;
        self.advance()?;

        let mut operands = Vec::new();

        // Parse operands if present
        if !matches!(self.current, Token::Newline | Token::Eof) {
            loop {
                operands.push(self.parse_operand()?);

                if matches!(self.current, Token::Comma) {
                    self.advance()?;
                } else {
                    break;
                }
            }
        }

        Ok(Instruction { mnemonic, operands })
    }

    fn parse_mnemonic(&self) -> Result<Mnemonic> {
        if let Token::Identifier(name) = &self.current {
            match name.to_uppercase().as_str() {
                "MOV" => Ok(Mnemonic::Mov),
                "MOVB" => Ok(Mnemonic::Movb),
                "CMP" => Ok(Mnemonic::Cmp),
                "CMPB" => Ok(Mnemonic::Cmpb),
                "BIT" => Ok(Mnemonic::Bit),
                "BITB" => Ok(Mnemonic::Bitb),
                "BIC" => Ok(Mnemonic::Bic),
                "BICB" => Ok(Mnemonic::Bicb),
                "BIS" => Ok(Mnemonic::Bis),
                "BISB" => Ok(Mnemonic::Bisb),
                "ADD" => Ok(Mnemonic::Add),
                "SUB" => Ok(Mnemonic::Sub),
                "CLR" => Ok(Mnemonic::Clr),
                "CLRB" => Ok(Mnemonic::Clrb),
                "COM" => Ok(Mnemonic::Com),
                "COMB" => Ok(Mnemonic::Comb),
                "INC" => Ok(Mnemonic::Inc),
                "INCB" => Ok(Mnemonic::Incb),
                "DEC" => Ok(Mnemonic::Dec),
                "DECB" => Ok(Mnemonic::Decb),
                "NEG" => Ok(Mnemonic::Neg),
                "NEGB" => Ok(Mnemonic::Negb),
                "TST" => Ok(Mnemonic::Tst),
                "TSTB" => Ok(Mnemonic::Tstb),
                "ASL" | "ASLB" => Ok(Mnemonic::Asl),
                "ASR" => Ok(Mnemonic::Asr),
                "ASRB" => Ok(Mnemonic::Asrb),
                "ROR" => Ok(Mnemonic::Ror),
                "RORB" => Ok(Mnemonic::Rorb),
                "ROL" => Ok(Mnemonic::Rol),
                "ROLB" => Ok(Mnemonic::Rolb),
                "SWAB" => Ok(Mnemonic::Swab),
                "ADC" => Ok(Mnemonic::Adc),
                "ADCB" => Ok(Mnemonic::Adcb),
                "SBC" => Ok(Mnemonic::Sbc),
                "SBCB" => Ok(Mnemonic::Sbcb),
                "JMP" => Ok(Mnemonic::Jmp),
                "BR" => Ok(Mnemonic::Br),
                "BNE" => Ok(Mnemonic::Bne),
                "BEQ" => Ok(Mnemonic::Beq),
                "BGE" => Ok(Mnemonic::Bge),
                "BLT" => Ok(Mnemonic::Blt),
                "BGT" => Ok(Mnemonic::Bgt),
                "BLE" => Ok(Mnemonic::Ble),
                "BPL" => Ok(Mnemonic::Bpl),
                "BMI" => Ok(Mnemonic::Bmi),
                "BHI" => Ok(Mnemonic::Bhi),
                "BLOS" => Ok(Mnemonic::Blos),
                "BVC" => Ok(Mnemonic::Bvc),
                "BVS" => Ok(Mnemonic::Bvs),
                "BCC" | "BHIS" => Ok(Mnemonic::Bcc),
                "BCS" | "BLO" => Ok(Mnemonic::Bcs),
                "JSR" => Ok(Mnemonic::Jsr),
                "RTS" => Ok(Mnemonic::Rts),
                "MUL" => Ok(Mnemonic::Mul),
                "DIV" => Ok(Mnemonic::Div),
                "ASH" => Ok(Mnemonic::Ash),
                "ASHC" => Ok(Mnemonic::Ashc),
                "XOR" => Ok(Mnemonic::Xor),
                "SOB" => Ok(Mnemonic::Sob),
                "HALT" => Ok(Mnemonic::Halt),
                "WAIT" => Ok(Mnemonic::Wait),
                "RTI" => Ok(Mnemonic::Rti),
                "IOT" => Ok(Mnemonic::Iot),
                "RESET" => Ok(Mnemonic::Reset),
                "NOP" => Ok(Mnemonic::Nop),
                "CLC" => Ok(Mnemonic::Clc),
                "SEC" => Ok(Mnemonic::Sec),
                "CLV" => Ok(Mnemonic::Clv),
                "SEV" => Ok(Mnemonic::Sev),
                "CLZ" => Ok(Mnemonic::Clz),
                "SEZ" => Ok(Mnemonic::Sez),
                "CLN" => Ok(Mnemonic::Cln),
                "SEN" => Ok(Mnemonic::Sen),
                "CCC" => Ok(Mnemonic::Ccc),
                "SCC" => Ok(Mnemonic::Scc),
                other => Err(AsmError::new(0, 0, format!("Unknown mnemonic: {other}"))),
            }
        } else {
            Err(AsmError::new(0, 0, "Expected mnemonic"))
        }
    }

    fn parse_operand(&mut self) -> Result<Operand> {
        // This is a placeholder - full implementation would handle all addressing modes
        todo!("Implement full operand parsing")
    }

    fn parse_directive(&mut self) -> Result<Directive> {
        // This is a placeholder - full implementation would handle all directives
        todo!("Implement directive parsing")
    }

    fn advance(&mut self) -> Result<()> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }
}
