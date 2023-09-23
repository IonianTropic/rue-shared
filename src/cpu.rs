pub const SHIFT_RD: Ilen = 7;
pub const SHIFT_FUNCT3: Ilen = 12;
pub const SHIFT_RS1: Ilen = 15;
pub const SHIFT_RS2: Ilen = 20;
pub const SHIFT_FUNCT7: Ilen = 25;

pub const MASK_OPCODE: Ilen = 0b1111111;
pub const MASK_RD: Ilen = 0b11111 << SHIFT_RD;
pub const MASK_FUNCT3: Ilen = 0b111 << SHIFT_FUNCT3;
pub const MASK_RS1: Ilen = 0b11111 << SHIFT_RS1;
pub const MASK_RS2: Ilen = 0b11111 << SHIFT_RS2;
pub const MASK_FUNCT7: Ilen = 0b1111111  << SHIFT_FUNCT7;
pub const MASK_I_IMM: Ilen = 0b111111111111 << SHIFT_RS2;

pub const OPCODE_OP: u32 = 0b_01_100_11;
pub const OPCODE_OP_IMM: u32 = 0b_00_100_11;
pub const OPCODE_LOAD: u32 = 0b_00_000_11;
pub const OPCODE_STORE: u32 = 0b_01_000_11;
pub const OPCODE_BRANCH: u32 = 0b_11_000_11;
pub const OPCODE_JAL: u32 = 0b_11_011_11;
pub const OPCODE_JALR: u32 = 0b_11_001_11;
pub const OPCODE_LUI: u32 = 0b_01_101_11;
pub const OPCODE_AUIPC: u32 = 0b_00_101_11;
pub const OPCODE_FENCE: u32 = 0b_00_011_11;
pub const OPCODE_SYSTEM: u32 = 0b_11_100_11;

pub const IALIGN: Xlen = 4;

pub type Xlen = u32; // Register length
pub type Ixlen = i32; // Signed register length
pub type Ialign = u32; // Min instruction length
pub type Ilen = u32; // Max instruction length
