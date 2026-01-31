use crate::error::{AsmError, Result};
use crate::{Instruction, Mnemonic, Operand};
use pdp11_common::Word;

/// Encode an instruction to machine code
pub fn encode_instruction(insn: &Instruction, _location: u16) -> Result<Vec<Word>> {
    let opcode = encode_opcode(&insn.mnemonic, &insn.operands)?;
    let mut words = vec![Word::from_u16(opcode)];

    // Add extra words for addressing modes
    for operand in &insn.operands {
        if operand.mode.needs_extra_word() {
            // TODO: Evaluate expression and add word
            // For now, just add placeholder
            words.push(Word::ZERO);
        }
    }

    Ok(words)
}

fn encode_opcode(mnemonic: &Mnemonic, operands: &[Operand]) -> Result<u16> {
    use Mnemonic::*;

    match mnemonic {
        // Double operand: opcode | src | dst
        Mov => encode_double_operand(0o010000, operands),
        Movb => encode_double_operand(0o110000, operands),
        Cmp => encode_double_operand(0o020000, operands),
        Cmpb => encode_double_operand(0o120000, operands),
        Bit => encode_double_operand(0o030000, operands),
        Bitb => encode_double_operand(0o130000, operands),
        Bic => encode_double_operand(0o040000, operands),
        Bicb => encode_double_operand(0o140000, operands),
        Bis => encode_double_operand(0o050000, operands),
        Bisb => encode_double_operand(0o150000, operands),
        Add => encode_double_operand(0o060000, operands),
        Sub => encode_double_operand(0o160000, operands),

        // Single operand: opcode | dst
        Clr => encode_single_operand(0o005000, operands),
        Clrb => encode_single_operand(0o105000, operands),
        Com => encode_single_operand(0o005100, operands),
        Comb => encode_single_operand(0o105100, operands),
        Inc => encode_single_operand(0o005200, operands),
        Incb => encode_single_operand(0o105200, operands),
        Dec => encode_single_operand(0o005300, operands),
        Decb => encode_single_operand(0o105300, operands),
        Neg => encode_single_operand(0o005400, operands),
        Negb => encode_single_operand(0o105400, operands),
        Adc => encode_single_operand(0o005500, operands),
        Adcb => encode_single_operand(0o105500, operands),
        Sbc => encode_single_operand(0o005600, operands),
        Sbcb => encode_single_operand(0o105600, operands),
        Tst => encode_single_operand(0o005700, operands),
        Tstb => encode_single_operand(0o105700, operands),
        Ror => encode_single_operand(0o006000, operands),
        Rorb => encode_single_operand(0o106000, operands),
        Rol => encode_single_operand(0o006100, operands),
        Rolb => encode_single_operand(0o106100, operands),
        Asr => encode_single_operand(0o006200, operands),
        Asrb => encode_single_operand(0o106200, operands),
        Asl => encode_single_operand(0o006300, operands),
        Aslb => encode_single_operand(0o106300, operands),
        Jmp => encode_single_operand(0o000100, operands),
        Swab => encode_single_operand(0o000300, operands),

        // Branch: opcode | offset
        Br => encode_branch(0o000400, operands),
        Bne => encode_branch(0o001000, operands),
        Beq => encode_branch(0o001400, operands),
        Bge => encode_branch(0o002000, operands),
        Blt => encode_branch(0o002400, operands),
        Bgt => encode_branch(0o003000, operands),
        Ble => encode_branch(0o003400, operands),
        Bpl => encode_branch(0o100000, operands),
        Bmi => encode_branch(0o100400, operands),
        Bhi => encode_branch(0o101000, operands),
        Blos => encode_branch(0o101400, operands),
        Bvc => encode_branch(0o102000, operands),
        Bvs => encode_branch(0o102400, operands),
        Bcc => encode_branch(0o103000, operands),
        Bcs => encode_branch(0o103400, operands),

        // No operand
        Halt => Ok(0o000000),
        Wait => Ok(0o000001),
        Rti => Ok(0o000002),
        Nop => Ok(0o000240),
        Reset => Ok(0o000005),
        Iot => Ok(0o000004),
        Clc => Ok(0o000241),
        Sec => Ok(0o000261),
        Clv => Ok(0o000242),
        Sev => Ok(0o000262),
        Clz => Ok(0o000244),
        Sez => Ok(0o000264),
        Cln => Ok(0o000250),
        Sen => Ok(0o000270),
        Ccc => Ok(0o000257),
        Scc => Ok(0o000277),

        // EIS and others - simplified for now
        _ => Err(AsmError::new(
            0,
            0,
            format!("Encoding not yet implemented for {mnemonic:?}"),
        )),
    }
}

fn encode_double_operand(base: u16, operands: &[Operand]) -> Result<u16> {
    if operands.len() != 2 {
        return Err(AsmError::new(
            0,
            0,
            format!("Expected 2 operands, got {}", operands.len()),
        ));
    }

    let src = operands[0].mode.encode();
    let dst = operands[1].mode.encode();

    Ok(base | (src << 6) | dst)
}

fn encode_single_operand(base: u16, operands: &[Operand]) -> Result<u16> {
    if operands.len() != 1 {
        return Err(AsmError::new(
            0,
            0,
            format!("Expected 1 operand, got {}", operands.len()),
        ));
    }

    let dst = operands[0].mode.encode();
    Ok(base | dst)
}

fn encode_branch(_base: u16, _operands: &[Operand]) -> Result<u16> {
    // TODO: Calculate offset
    todo!("Branch encoding")
}
