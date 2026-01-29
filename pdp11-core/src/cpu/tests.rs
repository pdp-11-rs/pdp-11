use super::*;
use std::io::Write;
use tempfile::NamedTempFile;

/// Helper to create a test CPU with a temporary RK image
fn create_test_cpu() -> Cpu {
    // Create a temporary file for the RK disk image
    let mut temp_file = NamedTempFile::new().unwrap();
    // Write a minimal disk image (at least 1 block = 512 bytes)
    temp_file.write_all(&[0u8; 512]).unwrap();
    temp_file.flush().unwrap();

    // Create CPU with the temporary RK image
    Cpu::new(temp_file.path()).unwrap()
}

#[test]
fn test_jmp_register_deferred() {
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
fn test_jmp_autoincrement() {
    let mut cpu = create_test_cpu();
    // Set up R2 to point to a memory location containing the target address
    cpu.registers[R2] = 0o2000.into();
    // Store target address 0o5000 at memory location 0o2000
    let addr = Address::<Word>::from_u16(0o2000);
    *cpu.ram.word_mut(addr) = 0o5000.into();

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
fn test_jmp_autoincrement_deferred() {
    let mut cpu = create_test_cpu();
    // Set up R3 to point to a memory location
    cpu.registers[R3] = 0o3000.into();
    // Store an address at 0o3000 that points to another address
    *cpu.ram.word_mut(Address::from_u16(0o3000)) = 0o4000.into();
    // Store the final target address at 0o4000
    *cpu.ram.word_mut(Address::from_u16(0o4000)) = 0o6000.into();

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
fn test_jmp_autodecrement() {
    let mut cpu = create_test_cpu();
    // Set up R4 to point just after the memory location containing target
    cpu.registers[R4] = 0o1002.into();
    // Store target address at 0o1000
    *cpu.ram.word_mut(Address::from_u16(0o1000)) = 0o7000.into();

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
#[ignore] // Index mode not yet implemented
fn test_jmp_index() {
    let mut cpu = create_test_cpu();
    // Set up PC for index mode (it will be used to read the index value)
    cpu.registers[PC] = 0o1000.into();
    // Store index offset at 0o1000
    *cpu.ram.word_mut(Address::from_u16(0o1000)) = 0o100.into();
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
#[ignore] // Index mode not yet implemented
fn test_jmp_pc_relative() {
    let mut cpu = create_test_cpu();
    // Set up PC
    cpu.registers[PC] = 0o1000.into();
    // Store offset at 0o1000 (PC-relative addressing)
    *cpu.ram.word_mut(Address::from_u16(0o1000)) = 0o500.into();

    // JMP X(PC) - PC-relative addressing
    let operand = Operand {
        mode: RegisterAddressingMode::Index,
        register: PC,
    };

    cpu.jmp(operand);

    // PC should be 0o1000 (old PC) + 2 (auto-increment) + 0o500 (offset) = 0o1502
    assert_eq!(cpu.registers[PC], 0o1502.into());
}
