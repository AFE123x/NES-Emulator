#[derive(Debug)]
pub enum Instruction {
    // Load/Store Operations
    LDA, LDX, LDY, STA, STX, STY,
    // Register Transfers
    TAX, TAY, TXA, TYA,
    // Stack Operations
    TSX, TXS, PHA, PHP, PLA, PLP,
    // Logical
    AND, EOR, ORA, BIT,
    // Arithmetic
    ADC, SBC, CMP, CPX, CPY,
    // Increments & Decrements
    INC, INX, INY, DEC, DEX, DEY,
    // Shifts
    ASL, LSR, ROL, ROR,
    // Jumps & Calls
    JMP, JSR, RTS,
    // Branches
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    // Status Flag Changes
    CLC, CLD, CLI, CLV, SEC, SED, SEI,
    // System Functions
    BRK, NOP, RTI,
    // illegal opcodes
    SRE, LAX, LAS, SHY, RRA, DCP, SAX,
    RLA, SLO, ISC, TAS, LXA, JAM, SHX,
    SBX, ANE, SHA, ARR, ALR, ANC,
}
#[derive(Debug)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IDX,
    IDY,
}