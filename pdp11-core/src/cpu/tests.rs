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

// Console I/O tests

#[test]
fn console_output() {
    let mut cpu = create_test_cpu();

    // Write character 'A' (0o101) to XBUF
    let xbuf_addr = 0o177566;
    cpu.registers[R0] = xbuf_addr.into();
    cpu.registers[R1] = 0o101.into(); // 'A'

    // MOV R1, (R0) - write to XBUF
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };

    cpu.mov(src, dst);

    // Verify XCSR shows ready
    let xcsr_addr = Address::<Word>::from_u16(0o177564);
    let xcsr = cpu.read_console(xcsr_addr);
    assert_eq!(xcsr.as_u16() & 0o200, 0o200); // XMIT_READY bit set
}

#[test]
fn console_input() {
    let mut cpu = create_test_cpu();

    // Simulate input character 'B' (0o102)
    cpu.console.input_char(b'B');

    // Check RCSR shows data available
    let rcsr_addr = Address::<Word>::from_u16(0o177560);
    let rcsr = cpu.read_console(rcsr_addr);
    assert_eq!(rcsr.as_u16() & 0o200, 0o200); // READER_DONE bit set

    // Read RBUF
    let rbuf_addr = Address::<Word>::from_u16(0o177562);
    let rbuf = cpu.read_console(rbuf_addr);
    assert_eq!(rbuf.as_u16() & 0o377, 0o102); // Character 'B'

    // After reading, RCSR DONE bit should be clear
    let rcsr = cpu.read_console(rcsr_addr);
    assert_eq!(rcsr.as_u16() & 0o200, 0); // READER_DONE bit clear
}

#[test]
fn console_mov_read() {
    let mut cpu = create_test_cpu();

    // Simulate input character 'C' (0o103)
    cpu.console.input_char(b'C');

    // MOV from RBUF to R2
    cpu.registers[R0] = 0o177562.into(); // RBUF address

    let src = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };

    cpu.mov(src, dst);

    // Check that R2 contains the character
    assert_eq!(cpu.registers[R2].as_u16() & 0o377, 0o103);
}

#[test]
fn test_jsr_basic() {
    let mut cpu = create_test_cpu();

    // Setup: PC at 0o1000, R5 contains 0o2000, SP at 0o10000
    cpu.registers[PC] = 0o1000.into();
    cpu.registers[R5] = 0o2000.into();
    cpu.registers[SP] = 0o10000.into(); // Stack pointer

    // JSR R5, (R0) where R0 points to 0o3000
    cpu.registers[R0] = 0o3000.into();
    let dst = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };
    cpu.jsr(R5, dst);

    // Check: SP decremented by 2, old R5 pushed to stack
    assert_eq!(cpu.registers[SP], 0o7776.into());
    assert_eq!(cpu.ram[Address::<Word>::from_u16(0o7776)], 0o2000.into());

    // Check: R5 now contains old PC
    assert_eq!(cpu.registers[R5], 0o1000.into());

    // Check: PC now points to destination
    assert_eq!(cpu.registers[PC], 0o3000.into());
}

#[test]
fn test_rts_basic() {
    let mut cpu = create_test_cpu();

    // Setup: Simulate state after JSR
    // PC should be wherever the subroutine is
    cpu.registers[PC] = 0o3000.into();
    // R5 contains the return address (old PC)
    cpu.registers[R5] = 0o1002.into();
    // SP points to saved R5 value on stack
    cpu.registers[SP] = 0o7776.into();
    cpu.ram[Address::<Word>::from_u16(0o7776)] = 0o2000.into(); // Old R5 value

    // RTS R5
    cpu.rts(R5);

    // Check: PC restored from R5
    assert_eq!(cpu.registers[PC], 0o1002.into());

    // Check: R5 restored from stack
    assert_eq!(cpu.registers[R5], 0o2000.into());

    // Check: SP incremented by 2
    assert_eq!(cpu.registers[SP], 0o10000.into());
}

#[test]
fn test_jsr_rts_roundtrip() {
    let mut cpu = create_test_cpu();

    // Setup initial state
    cpu.registers[PC] = 0o1000.into();
    cpu.registers[R5] = 0o5555.into();
    cpu.registers[SP] = 0o10000.into();

    // Save initial values
    let initial_pc = cpu.registers[PC];
    let initial_r5 = cpu.registers[R5];
    let initial_sp = cpu.registers[SP];

    // JSR R5, #3000 (using immediate mode)
    cpu.registers[R0] = 0o3000.into();
    let dst = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };
    cpu.jsr(R5, dst);

    // Verify JSR worked
    assert_eq!(cpu.registers[PC], 0o3000.into());
    assert_eq!(cpu.registers[R5], initial_pc);

    // Now RTS to return
    cpu.rts(R5);

    // Verify we're back to initial state
    assert_eq!(cpu.registers[PC], initial_pc);
    assert_eq!(cpu.registers[R5], initial_r5);
    assert_eq!(cpu.registers[SP], initial_sp);
}

#[test]
fn test_nested_jsr() {
    let mut cpu = create_test_cpu();

    // Initial state
    cpu.registers[PC] = 0o1000.into();
    cpu.registers[R5] = 0o100.into();
    cpu.registers[R4] = 0o200.into();
    cpu.registers[SP] = 0o10000.into();

    let initial_r5 = cpu.registers[R5];
    let initial_r4 = cpu.registers[R4];

    // First JSR R5, #2000
    cpu.registers[R0] = 0o2000.into();
    let dst1 = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };
    cpu.jsr(R5, dst1);

    // Simulate subroutine updating PC
    cpu.registers[PC] = 0o2004.into();

    // Second JSR R4, #3000 (nested call)
    cpu.registers[R0] = 0o3000.into();
    let dst2 = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };
    cpu.jsr(R4, dst2);

    // Verify stack has both frames
    // SP should be at 0o10000 - 4 = 0o7774
    assert_eq!(cpu.registers[SP], 0o7774.into());
    // First saved R5 at 0o7776
    assert_eq!(cpu.ram[Address::<Word>::from_u16(0o7776)], initial_r5);
    // Second saved R4 at 0o7774
    assert_eq!(cpu.ram[Address::<Word>::from_u16(0o7774)], initial_r4);

    // Return from inner subroutine
    cpu.rts(R4);
    assert_eq!(cpu.registers[PC], 0o2004.into());
    assert_eq!(cpu.registers[R4], initial_r4);

    // Return from outer subroutine
    cpu.rts(R5);
    assert_eq!(cpu.registers[PC], 0o1000.into());
    assert_eq!(cpu.registers[R5], initial_r5);
    assert_eq!(cpu.registers[SP], 0o10000.into());
}

#[test]
fn test_jsr_with_different_addressing_modes() {
    let mut cpu = create_test_cpu();

    cpu.registers[PC] = 0o1000.into();
    cpu.registers[R5] = 0o5555.into();
    cpu.registers[SP] = 0o10000.into();
    cpu.registers[R1] = 0o4000.into();

    // JSR R5, @(R1)+ - autoincrement deferred
    cpu.ram[Address::<Word>::from_u16(0o4000)] = 0o3000.into(); // Address to jump to
    let dst = Operand {
        mode: RegisterAddressingMode::AutoincrementDeferred,
        register: R1,
    };
    cpu.jsr(R5, dst);

    // Check R1 was incremented
    assert_eq!(cpu.registers[R1], 0o4002.into());
    // Check we jumped to the right place
    assert_eq!(cpu.registers[PC], 0o3000.into());
    // Check R5 saved old PC
    assert_eq!(cpu.registers[R5], 0o1000.into());
}

#[test]
fn test_stack_grows_downward() {
    let mut cpu = create_test_cpu();

    cpu.registers[SP] = 0o10000.into();
    cpu.registers[R5] = 0o1234.into();
    cpu.registers[R4] = 0o5670.into();

    let initial_sp = cpu.registers[SP];

    // First JSR
    cpu.registers[R0] = 0o2000.into();
    let dst = Operand {
        mode: RegisterAddressingMode::RegisterDeferred,
        register: R0,
    };
    cpu.jsr(R5, dst);

    // SP should have decreased
    assert!(cpu.registers[SP] < initial_sp);
    let sp_after_first = cpu.registers[SP];

    // Second JSR
    cpu.registers[R0] = 0o3000.into();
    cpu.jsr(R4, dst);

    // SP should have decreased more
    assert!(cpu.registers[SP] < sp_after_first);

    // Return from both
    cpu.rts(R4);
    assert_eq!(cpu.registers[SP], sp_after_first);
    cpu.rts(R5);
    assert_eq!(cpu.registers[SP], initial_sp);
}

// ===== Single Operand Instruction Tests =====

#[test]
fn com_basic() {
    let mut cpu = create_test_cpu();
    cpu.registers[R0] = Word::from_u16(0o125252);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.com(dst);
    assert_eq!(cpu.registers[R0], Word::from_u16(0o052525));
    assert!(!cpu.psw[N]); // Result is positive
    assert!(!cpu.psw[Z]); // Result is non-zero
    assert!(!cpu.psw[V]); // V always cleared
    assert!(cpu.psw[C]); // C always set
}

#[test]
fn com_zero() {
    let mut cpu = create_test_cpu();
    cpu.registers[R1] = Word::from_u16(0o177777);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    cpu.com(dst);
    assert_eq!(cpu.registers[R1], Word::zero());
    assert!(!cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(!cpu.psw[V]);
    assert!(cpu.psw[C]);
}

#[test]
fn inc_basic() {
    let mut cpu = create_test_cpu();
    cpu.registers[R2] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.inc(dst);
    assert_eq!(cpu.registers[R2], Word::from_u16(101));
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn inc_overflow() {
    let mut cpu = create_test_cpu();
    cpu.registers[R3] = Word::from_u16(0o077777); // Max positive value
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R3,
    };
    cpu.inc(dst);
    assert_eq!(cpu.registers[R3], Word::from_u16(0o100000)); // Becomes negative
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[V]); // Overflow detected
}

#[test]
fn dec_basic() {
    let mut cpu = create_test_cpu();
    cpu.registers[R4] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R4,
    };
    cpu.dec(dst);
    assert_eq!(cpu.registers[R4], Word::from_u16(99));
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn dec_overflow() {
    let mut cpu = create_test_cpu();
    cpu.registers[R5] = Word::from_u16(0o100000); // Min negative value
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R5,
    };
    cpu.dec(dst);
    assert_eq!(cpu.registers[R5], Word::from_u16(0o077777)); // Becomes positive
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[V]); // Overflow detected
}

#[test]
fn neg_basic() {
    let mut cpu = create_test_cpu();
    cpu.registers[R0] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.neg(dst);
    assert_eq!(cpu.registers[R0].as_u16(), 0u16.wrapping_sub(100));
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[C]);
}

#[test]
fn neg_zero() {
    let mut cpu = create_test_cpu();
    cpu.registers[R1] = Word::zero();
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    cpu.neg(dst);
    assert_eq!(cpu.registers[R1], Word::zero());
    assert!(!cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(!cpu.psw[C]); // C clear when result is zero
}

#[test]
fn neg_overflow() {
    let mut cpu = create_test_cpu();
    cpu.registers[R2] = Word::from_u16(0o100000); // Min negative value
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.neg(dst);
    assert_eq!(cpu.registers[R2], Word::from_u16(0o100000)); // Stays same
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[V]); // Overflow
    assert!(cpu.psw[C]);
}

#[test]
fn adc_no_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = false;
    cpu.registers[R3] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R3,
    };
    cpu.adc(dst);
    assert_eq!(cpu.registers[R3], Word::from_u16(100)); // Unchanged
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
    assert!(!cpu.psw[C]);
}

#[test]
fn adc_with_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.registers[R4] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R4,
    };
    cpu.adc(dst);
    assert_eq!(cpu.registers[R4], Word::from_u16(101));
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
    assert!(!cpu.psw[C]);
}

#[test]
fn adc_overflow() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.registers[R5] = Word::from_u16(0o077777); // Max positive
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R5,
    };
    cpu.adc(dst);
    assert_eq!(cpu.registers[R5], Word::from_u16(0o100000));
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[V]); // Overflow
}

#[test]
fn sbc_no_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = false;
    cpu.registers[R0] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.sbc(dst);
    assert_eq!(cpu.registers[R0], Word::from_u16(100)); // Unchanged
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
    assert!(!cpu.psw[C]);
}

#[test]
fn sbc_with_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.registers[R1] = Word::from_u16(100);
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    cpu.sbc(dst);
    assert_eq!(cpu.registers[R1], Word::from_u16(99));
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
    assert!(!cpu.psw[C]);
}

#[test]
fn sbc_underflow() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.registers[R2] = Word::zero();
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.sbc(dst);
    assert_eq!(cpu.registers[R2], Word::from_u16(0o177777));
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[C]); // Borrow
}

#[test]
fn ror_basic() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = false;
    cpu.registers[R3] = Word::from_u16(0b0000_0000_0000_0110); // 6
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R3,
    };
    cpu.ror(dst);
    assert_eq!(cpu.registers[R3], Word::from_u16(0b0000_0000_0000_0011)); // 3
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[C]); // Bit 0 was 0
}

#[test]
fn ror_with_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.registers[R4] = Word::from_u16(0b0000_0000_0000_0100); // 4
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R4,
    };
    cpu.ror(dst);
    assert_eq!(cpu.registers[R4], Word::from_u16(0b1000_0000_0000_0010)); // Bit 15 set from carry
    assert!(cpu.psw[N]); // Bit 15 is now set
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[C]); // Bit 0 was 0
}

#[test]
fn rol_basic() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = false;
    cpu.registers[R5] = Word::from_u16(0b0100_0000_0000_0000); // Bit 14 set
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R5,
    };
    cpu.rol(dst);
    assert_eq!(cpu.registers[R5], Word::from_u16(0b1000_0000_0000_0000)); // Bit 15 set
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[C]); // Bit 15 was 0
}

#[test]
fn rol_with_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.registers[R0] = Word::from_u16(0b1000_0000_0000_0010); // Bits 15 and 1 set
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.rol(dst);
    assert_eq!(cpu.registers[R0], Word::from_u16(0b0000_0000_0000_0101)); // Bit 0 set from carry
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[C]); // Bit 15 was 1
}

#[test]
fn asr_positive() {
    let mut cpu = create_test_cpu();
    cpu.registers[R1] = Word::from_u16(0b0000_0000_0000_0110); // 6
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    cpu.asr(dst);
    assert_eq!(cpu.registers[R1], Word::from_u16(0b0000_0000_0000_0011)); // 3, sign extended (0)
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[C]); // Bit 0 was 0
}

#[test]
fn asr_negative() {
    let mut cpu = create_test_cpu();
    cpu.registers[R2] = Word::from_u16(0b1000_0000_0000_0110); // Negative number
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.asr(dst);
    assert_eq!(cpu.registers[R2], Word::from_u16(0b1100_0000_0000_0011)); // Sign extended (1)
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[C]); // Bit 0 was 0
}

#[test]
fn asr_with_carry_out() {
    let mut cpu = create_test_cpu();
    cpu.registers[R3] = Word::from_u16(0b1000_0000_0000_0111); // Negative, bit 0 set
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R3,
    };
    cpu.asr(dst);
    assert_eq!(cpu.registers[R3], Word::from_u16(0b1100_0000_0000_0011)); // Sign extended
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[C]); // Bit 0 was 1
}

// ===== BIC/BIS Instruction Tests =====

#[test]
fn bic_basic() {
    let mut cpu = create_test_cpu();
    cpu.registers[R0] = Word::from_u16(0o177777); // All bits set
    cpu.registers[R1] = Word::from_u16(0o000017); // Bits 0-3 set
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bic(src, dst);
    // Should clear bits 0-3 in R0
    assert_eq!(cpu.registers[R0], Word::from_u16(0o177760));
    assert!(cpu.psw[N]); // Result is negative
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]); // V always cleared
}

#[test]
fn bic_clear_all() {
    let mut cpu = create_test_cpu();
    cpu.registers[R2] = Word::from_u16(0o125252);
    cpu.registers[R3] = Word::from_u16(0o177777); // Clear all
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R3,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.bic(src, dst);
    assert_eq!(cpu.registers[R2], Word::zero());
    assert!(!cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn bic_no_effect() {
    let mut cpu = create_test_cpu();
    cpu.registers[R4] = Word::from_u16(0o125252);
    cpu.registers[R5] = Word::from_u16(0o052525); // Non-overlapping bits
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R5,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R4,
    };
    cpu.bic(src, dst);
    assert_eq!(cpu.registers[R4], Word::from_u16(0o125252)); // Unchanged
    assert!(cpu.psw[N]); // 0o125252 has bit 15 set
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn bic_single_bit() {
    let mut cpu = create_test_cpu();
    cpu.registers[R0] = Word::from_u16(0o100000); // Bit 15 set (negative)
    cpu.registers[R1] = Word::from_u16(0o100000); // Clear bit 15
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bic(src, dst);
    assert_eq!(cpu.registers[R0], Word::zero());
    assert!(!cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn bis_basic() {
    let mut cpu = create_test_cpu();
    cpu.registers[R0] = Word::from_u16(0o177760); // Bits 4-15 set
    cpu.registers[R1] = Word::from_u16(0o000017); // Bits 0-3 set
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bis(src, dst);
    // Should set bits 0-3 in R0
    assert_eq!(cpu.registers[R0], Word::from_u16(0o177777));
    assert!(cpu.psw[N]); // Result is negative
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]); // V always cleared
}

#[test]
fn bis_all_zero() {
    let mut cpu = create_test_cpu();
    cpu.registers[R2] = Word::zero();
    cpu.registers[R3] = Word::zero();
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R3,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.bis(src, dst);
    assert_eq!(cpu.registers[R2], Word::zero());
    assert!(!cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn bis_no_effect() {
    let mut cpu = create_test_cpu();
    cpu.registers[R4] = Word::from_u16(0o177777); // All bits set
    cpu.registers[R5] = Word::from_u16(0o125252);
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R5,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R4,
    };
    cpu.bis(src, dst);
    assert_eq!(cpu.registers[R4], Word::from_u16(0o177777)); // Still all set
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn bis_set_negative_bit() {
    let mut cpu = create_test_cpu();
    cpu.registers[R0] = Word::from_u16(0o077777); // Positive (bit 15 clear)
    cpu.registers[R1] = Word::from_u16(0o100000); // Set bit 15
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bis(src, dst);
    assert_eq!(cpu.registers[R0], Word::from_u16(0o177777));
    assert!(cpu.psw[N]); // Now negative
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
}

#[test]
fn bic_bis_inverse_operations() {
    let mut cpu = create_test_cpu();
    let original = Word::from_u16(0o125252);
    let mask = Word::from_u16(0o070707);

    // Set up
    cpu.registers[R0] = original;
    cpu.registers[R1] = mask;
    cpu.registers[R2] = original;

    // BIC - clear bits where mask is set
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bic(src, dst);

    // BIS - set bits where mask is set
    let dst2 = Operand {
        mode: RegisterAddressingMode::Register,
        register: R2,
    };
    cpu.bis(src, dst2);

    // After BIC, the masked bits should be clear
    assert_eq!(
        cpu.registers[R0],
        Word::from_u16(original.as_u16() & !mask.as_u16())
    );

    // After BIS, the masked bits should be set
    assert_eq!(
        cpu.registers[R2],
        Word::from_u16(original.as_u16() | mask.as_u16())
    );
}

#[test]
fn bic_carry_unaffected() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true; // Set carry flag
    cpu.registers[R0] = Word::from_u16(0o177777);
    cpu.registers[R1] = Word::from_u16(0o000001);
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bic(src, dst);
    assert!(cpu.psw[C]); // Carry should be unaffected
}

#[test]
fn bis_carry_unaffected() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = false; // Clear carry flag
    cpu.registers[R0] = Word::zero();
    cpu.registers[R1] = Word::from_u16(0o000001);
    let src = Operand {
        mode: RegisterAddressingMode::Register,
        register: R1,
    };
    let dst = Operand {
        mode: RegisterAddressingMode::Register,
        register: R0,
    };
    cpu.bis(src, dst);
    assert!(!cpu.psw[C]); // Carry should be unaffected
}

// ===== PSW Flag Manipulation Instruction Tests =====

#[test]
fn nop_no_effect() {
    let mut cpu = create_test_cpu();
    // Set up some state
    cpu.registers[R0] = Word::from_u16(0o123456);
    cpu.psw[N] = true;
    cpu.psw[Z] = false;
    cpu.psw[V] = true;
    cpu.psw[C] = false;

    // Execute NOP
    cpu.execute(Word::from_u16(0o000240));

    // Everything should be unchanged
    assert_eq!(cpu.registers[R0], Word::from_u16(0o123456));
    assert!(cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(cpu.psw[V]);
    assert!(!cpu.psw[C]);
}

#[test]
fn clc_clears_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = true;
    cpu.execute(Word::from_u16(0o000241));
    assert!(!cpu.psw[C]);
}

#[test]
fn sec_sets_carry() {
    let mut cpu = create_test_cpu();
    cpu.psw[C] = false;
    cpu.execute(Word::from_u16(0o000261));
    assert!(cpu.psw[C]);
}

#[test]
fn clv_clears_overflow() {
    let mut cpu = create_test_cpu();
    cpu.psw[V] = true;
    cpu.execute(Word::from_u16(0o000242));
    assert!(!cpu.psw[V]);
}

#[test]
fn sev_sets_overflow() {
    let mut cpu = create_test_cpu();
    cpu.psw[V] = false;
    cpu.execute(Word::from_u16(0o000262));
    assert!(cpu.psw[V]);
}

#[test]
fn clz_clears_zero() {
    let mut cpu = create_test_cpu();
    cpu.psw[Z] = true;
    cpu.execute(Word::from_u16(0o000244));
    assert!(!cpu.psw[Z]);
}

#[test]
fn sez_sets_zero() {
    let mut cpu = create_test_cpu();
    cpu.psw[Z] = false;
    cpu.execute(Word::from_u16(0o000264));
    assert!(cpu.psw[Z]);
}

#[test]
fn cln_clears_negative() {
    let mut cpu = create_test_cpu();
    cpu.psw[N] = true;
    cpu.execute(Word::from_u16(0o000250));
    assert!(!cpu.psw[N]);
}

#[test]
fn sen_sets_negative() {
    let mut cpu = create_test_cpu();
    cpu.psw[N] = false;
    cpu.execute(Word::from_u16(0o000270));
    assert!(cpu.psw[N]);
}

#[test]
fn ccc_clears_all_flags() {
    let mut cpu = create_test_cpu();
    cpu.psw[N] = true;
    cpu.psw[Z] = true;
    cpu.psw[V] = true;
    cpu.psw[C] = true;
    cpu.execute(Word::from_u16(0o000257));
    assert!(!cpu.psw[N]);
    assert!(!cpu.psw[Z]);
    assert!(!cpu.psw[V]);
    assert!(!cpu.psw[C]);
}

#[test]
fn scc_sets_all_flags() {
    let mut cpu = create_test_cpu();
    cpu.psw[N] = false;
    cpu.psw[Z] = false;
    cpu.psw[V] = false;
    cpu.psw[C] = false;
    cpu.execute(Word::from_u16(0o000277));
    assert!(cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(cpu.psw[V]);
    assert!(cpu.psw[C]);
}

#[test]
fn flag_instructions_only_affect_target_flag() {
    let mut cpu = create_test_cpu();
    // Set all flags
    cpu.psw[N] = true;
    cpu.psw[Z] = true;
    cpu.psw[V] = true;
    cpu.psw[C] = true;

    // CLC should only clear C
    cpu.execute(Word::from_u16(0o000241));
    assert!(cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(cpu.psw[V]);
    assert!(!cpu.psw[C]);

    // CLN should only clear N
    cpu.execute(Word::from_u16(0o000250));
    assert!(!cpu.psw[N]);
    assert!(cpu.psw[Z]);
    assert!(cpu.psw[V]);
    assert!(!cpu.psw[C]);
}
