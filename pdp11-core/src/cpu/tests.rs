use super::*;

/// Helper to create a test CPU with an empty RK disk
fn create_test_cpu() -> Cpu {
    Cpu::for_testing()
}

#[test]
fn jmp_register_deferred() {
    let mut cpu = create_test_cpu();
    // Set up R1 to point to address 0o1000
    cpu.registers[R1] = 0o1000.into();

    // JMP (R1) - opcode 0o000110 (mode=1, reg=1)
    let operand = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R1,
    };

    cpu.jmp(operand);

    // PC should now be 0o1000
    assert_eq!(cpu.registers[PC], 0o1000.into());
}

#[test]
fn jmp_autoincrement() {
    let mut cpu = create_test_cpu();
    // Set up R2 to point to a memory location containing the target address
    cpu.registers[R2] = 0o2000.into();
    // Store target address 0o5000 at memory location 0o2000
    let addr = Address::<Word>::from_u16(0o2000);
    cpu.ram[addr] = Word::from(0o5000);

    // JMP (R2)+ - opcode 0o000122 (mode=2, reg=2)
    let operand = Operand {
        mode: RegisterAddressingMode::Autoincrement,
        register: R2,
    };

    cpu.jmp(operand);

    // PC should be set to the value read from memory
    assert_eq!(cpu.registers[PC], 0o5000.into());
    // R2 should be incremented by 2
    assert_eq!(cpu.registers[R2], 0o2002.into());
}

#[test]
fn jmp_autoincrement_deferred() {
    let mut cpu = create_test_cpu();
    // Set up R3 to point to a memory location
    cpu.registers[R3] = 0o3000.into();
    // Store an address at 0o3000 that points to another address
    cpu.ram[Address::<Word>::from_u16(0o3000)] = Word::from(0o4000);
    // Store the final target address at 0o4000
    cpu.ram[Address::<Word>::from_u16(0o4000)] = Word::from(0o6000);

    // JMP @(R3)+ - opcode 0o000133 (mode=3, reg=3)
    let operand = Operand {
        mode: RegisterAddressingMode::AutoincrementDeferred,
        register: R3,
    };

    cpu.jmp(operand);

    // PC should be set to the final target
    assert_eq!(cpu.registers[PC], 0o6000.into());
    // R3 should be incremented
    assert_eq!(cpu.registers[R3], 0o3002.into());
}

#[test]
fn jmp_autodecrement() {
    let mut cpu = create_test_cpu();
    // Set up R4 to point just after the memory location containing target
    cpu.registers[R4] = 0o1002.into();
    // Store target address at 0o1000
    cpu.ram[Address::<Word>::from_u16(0o1000)] = Word::from(0o7000);

    // JMP -(R4) - opcode 0o000144 (mode=4, reg=4)
    let operand = Operand {
        mode: RegisterAddressingMode::Autodecrement,
        register: R4,
    };

    cpu.jmp(operand);

    // PC should be set to the target
    assert_eq!(cpu.registers[PC], 0o7000.into());
    // R4 should be decremented by 2
    assert_eq!(cpu.registers[R4], 0o1000.into());
}

#[test]
fn jmp_index() {
    let mut cpu = create_test_cpu();
    // Set up PC for index mode (it will be used to read the index value)
    cpu.registers[PC] = 0o1000.into();
    // Store index offset at 0o1000
    cpu.ram[Address::<Word>::from_u16(0o1000)] = Word::from(0o100);
    // Set up R5 as base
    cpu.registers[R5] = 0o2000.into();

    // JMP X(R5) - opcode 0o000165 (mode=6, reg=5)
    let operand = Operand {
        mode: RegisterAddressingMode::Index,
        register: R5,
    };

    cpu.jmp(operand);

    // PC should be set to R5 + index = 0o2000 + 0o100 = 0o2100
    assert_eq!(cpu.registers[PC], 0o2100.into());
}

#[test]
fn jmp_pc_relative() {
    let mut cpu = create_test_cpu();
    // Set up PC
    cpu.registers[PC] = 0o1000.into();
    // Store offset at 0o1000 (PC-relative addressing)
    cpu.ram[Address::<Word>::from_u16(0o1000)] = Word::from(0o500);

    // JMP X(PC) - PC-relative addressing
    let operand = Operand {
        mode: RegisterAddressingMode::Index,
        register: PC,
    };

    cpu.jmp(operand);

    // PC should be 0o1000 (old PC) + 2 (auto-increment) + 0o500 (offset) = 0o1502
    assert_eq!(cpu.registers[PC], 0o1502.into());
}

#[test]
fn ram_index_notation() {
    let mut cpu = create_test_cpu();

    // Test Word indexing
    let addr = Address::<Word>::from_u16(0o1000);
    cpu.ram[addr] = 0o5432.into();
    assert_eq!(cpu.ram[addr].as_u16(), 0o5432);

    // Test Byte indexing
    let byte_addr = Address::<Byte>::from_u16(0o2000);
    cpu.ram[byte_addr] = 0o123.into();
    assert_eq!(cpu.ram[byte_addr].as_u8(), 0o123);
}

// Branch instruction tests

#[test]
fn br_forward() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o1000.into();

    // BR with offset +10 (in words, so +20 bytes)
    cpu.br(Offset(10));

    assert_eq!(cpu.registers[PC], 0o1024.into());
}

#[test]
fn br_backward() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o1000.into();

    // BR with offset -10 (in words, so -20 bytes)
    cpu.br(Offset(-10));

    assert_eq!(cpu.registers[PC], 0o0754.into());
}

#[test]
fn bne_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o2000.into();
    cpu.psw[Z] = false; // Not zero

    cpu.bne(Offset(5));

    assert_eq!(cpu.registers[PC], 0o2012.into());
}

#[test]
fn bne_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o2000.into();
    cpu.psw[Z] = true; // Zero

    cpu.bne(Offset(5));

    assert_eq!(cpu.registers[PC], 0o2000.into()); // Unchanged
}

#[test]
fn beq_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o3000.into();
    cpu.psw[Z] = true; // Zero

    cpu.beq(Offset(3));

    assert_eq!(cpu.registers[PC], 0o3006.into());
}

#[test]
fn beq_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o3000.into();
    cpu.psw[Z] = false; // Not zero

    cpu.beq(Offset(3));

    assert_eq!(cpu.registers[PC], 0o3000.into()); // Unchanged
}

#[test]
fn bpl_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o4000.into();
    cpu.psw[N] = false; // Positive

    cpu.bpl(Offset(7));

    assert_eq!(cpu.registers[PC], 0o4016.into());
}

#[test]
fn bpl_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o4000.into();
    cpu.psw[N] = true; // Negative

    cpu.bpl(Offset(7));

    assert_eq!(cpu.registers[PC], 0o4000.into()); // Unchanged
}

#[test]
fn bmi_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o5000.into();
    cpu.psw[N] = true; // Negative

    cpu.bmi(Offset(2));

    assert_eq!(cpu.registers[PC], 0o5004.into());
}

#[test]
fn bmi_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o5000.into();
    cpu.psw[N] = false; // Positive

    cpu.bmi(Offset(2));

    assert_eq!(cpu.registers[PC], 0o5000.into()); // Unchanged
}

#[test]
fn bvc_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o6000.into();
    cpu.psw[V] = false; // No overflow

    cpu.bvc(Offset(4));

    assert_eq!(cpu.registers[PC], 0o6010.into());
}

#[test]
fn bvc_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o6000.into();
    cpu.psw[V] = true; // Overflow

    cpu.bvc(Offset(4));

    assert_eq!(cpu.registers[PC], 0o6000.into()); // Unchanged
}

#[test]
fn bvs_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o7000.into();
    cpu.psw[V] = true; // Overflow

    cpu.bvs(Offset(1));

    assert_eq!(cpu.registers[PC], 0o7002.into());
}

#[test]
fn bvs_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o7000.into();
    cpu.psw[V] = false; // No overflow

    cpu.bvs(Offset(1));

    assert_eq!(cpu.registers[PC], 0o7000.into()); // Unchanged
}

#[test]
fn bcc_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o1234.into();
    cpu.psw[C] = false; // No carry

    cpu.bcc(Offset(8));

    assert_eq!(cpu.registers[PC], 0o1254.into());
}

#[test]
fn bcc_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o1234.into();
    cpu.psw[C] = true; // Carry

    cpu.bcc(Offset(8));

    assert_eq!(cpu.registers[PC], 0o1234.into()); // Unchanged
}

#[test]
fn bcs_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o2345.into();
    cpu.psw[C] = true; // Carry

    cpu.bcs(Offset(6));

    assert_eq!(cpu.registers[PC], 0o2361.into());
}

#[test]
fn bcs_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o2345.into();
    cpu.psw[C] = false; // No carry

    cpu.bcs(Offset(6));

    assert_eq!(cpu.registers[PC], 0o2345.into()); // Unchanged
}

#[test]
fn bge_taken_both_positive() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o3456.into();
    cpu.psw[N] = false;
    cpu.psw[V] = false; // N xor V = 0

    cpu.bge(Offset(3));

    assert_eq!(cpu.registers[PC], 0o3464.into());
}

#[test]
fn bge_taken_both_set() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o3456.into();
    cpu.psw[N] = true;
    cpu.psw[V] = true; // N xor V = 0

    cpu.bge(Offset(3));

    assert_eq!(cpu.registers[PC], 0o3464.into());
}

#[test]
fn bge_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o3456.into();
    cpu.psw[N] = true;
    cpu.psw[V] = false; // N xor V = 1

    cpu.bge(Offset(3));

    assert_eq!(cpu.registers[PC], 0o3456.into()); // Unchanged
}

#[test]
fn blt_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o4567.into();
    cpu.psw[N] = true;
    cpu.psw[V] = false; // N xor V = 1

    cpu.blt(Offset(2));

    assert_eq!(cpu.registers[PC], 0o4573.into());
}

#[test]
fn blt_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o4567.into();
    cpu.psw[N] = false;
    cpu.psw[V] = false; // N xor V = 0

    cpu.blt(Offset(2));

    assert_eq!(cpu.registers[PC], 0o4567.into()); // Unchanged
}

#[test]
fn bgt_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o5670.into();
    cpu.psw[Z] = false;
    cpu.psw[N] = false;
    cpu.psw[V] = false; // Z=0 and N xor V = 0

    cpu.bgt(Offset(5));

    assert_eq!(cpu.registers[PC], 0o5702.into());
}

#[test]
fn bgt_not_taken_zero() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o5670.into();
    cpu.psw[Z] = true; // Zero flag set
    cpu.psw[N] = false;
    cpu.psw[V] = false;

    cpu.bgt(Offset(5));

    assert_eq!(cpu.registers[PC], 0o5670.into()); // Unchanged
}

#[test]
fn bgt_not_taken_less() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o5670.into();
    cpu.psw[Z] = false;
    cpu.psw[N] = true;
    cpu.psw[V] = false; // N xor V = 1 (less than)

    cpu.bgt(Offset(5));

    assert_eq!(cpu.registers[PC], 0o5670.into()); // Unchanged
}

#[test]
fn ble_taken_zero() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o6701.into();
    cpu.psw[Z] = true; // Zero
    cpu.psw[N] = false;
    cpu.psw[V] = false;

    cpu.ble(Offset(4));

    assert_eq!(cpu.registers[PC], 0o6711.into());
}

#[test]
fn ble_taken_less() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o6701.into();
    cpu.psw[Z] = false;
    cpu.psw[N] = true;
    cpu.psw[V] = false; // N xor V = 1 (less than)

    cpu.ble(Offset(4));

    assert_eq!(cpu.registers[PC], 0o6711.into());
}

#[test]
fn ble_not_taken() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o6701.into();
    cpu.psw[Z] = false;
    cpu.psw[N] = false;
    cpu.psw[V] = false; // Greater than

    cpu.ble(Offset(4));

    assert_eq!(cpu.registers[PC], 0o6701.into()); // Unchanged
}

#[test]
fn branch_backward_wrapping() {
    let mut cpu = create_test_cpu();
    cpu.registers[PC] = 0o10.into();

    // Branch backward by 10 words = 20 bytes
    cpu.br(Offset(-10));

    // Should wrap around: 0o10 - 0o24 = 0o177764
    assert_eq!(cpu.registers[PC], 0o177764.into());
}

#[test]
fn branch_decode_opcode() {
    // Test that opcodes decode to correct branch instructions
    let br_opcode = Word::from(0o000400); // BR
    let bne_opcode = Word::from(0o001000); // BNE
    let beq_opcode = Word::from(0o001400); // BEQ
    let bpl_opcode = Word::from(0o100000); // BPL

    assert!(matches!(Instruction::from(br_opcode), Instruction::Br(_)));
    assert!(matches!(Instruction::from(bne_opcode), Instruction::Bne(_)));
    assert!(matches!(Instruction::from(beq_opcode), Instruction::Beq(_)));
    assert!(matches!(Instruction::from(bpl_opcode), Instruction::Bpl(_)));
}
