use super::*;

const BOOTROM: &[u16] = &[
    // (PAL-11 assembly)
    0o042113, // "KD"
    0o012706, // MOV #boot_start, SP
    0o2000,   //
    0o012700, // MOV #unit, R0        ; unit number
    0o000000, //
    0o010003, // MOV R0, R3
    0o000303, // SWAB R3
    0o006303, // ASL R3
    0o006303, // ASL R3
    0o006303, // ASL R3
    0o006303, // ASL R3
    0o006303, // ASL R3
    0o012701, // MOV #RKDA, R1        ; csr
    0o177412, //
    0o010311, // MOV R3, (R1)         ; load da
    0o005041, // CLR -(R1)            ; clear ba
    0o012741, // MOV #-256.*2, -(R1)  ; load wc
    0o177000, //
    0o012741, // MOV #READ+GO, -(R1)  ; read & go
    0o000005, //
    0o005002, // CLR R2
    0o005003, // CLR R3
    0o012704, // MOV #START+20, R4
    0o2020,   //
    0o005005, // CLR R5
    0o105711, // TSTB (R1)
    0o100376, // BPL .-2
    0o105011, // CLRB (R1)
    0o005007, // CLR PC
];

const BOOTROM_START: u16 = 0o2000;

impl Cpu {
    pub fn bootrom(&mut self) {
        self.registers[PC] = BOOTROM_START.into();
        let dst = Operand::pc();
        for word in BOOTROM {
            let data = Word::from(*word);
            self.store(dst, data);
        }
        self.registers[PC] = (BOOTROM_START + 2).into();
    }
}
