use crate::error::VMError;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    // Arithmetic 0x00 - 0x0F
    Add = 0x00,    // -- adds
    Sub = 0x01,    // -- subs
    Mul = 0x02,    // -- muls
    Div = 0x03,    // -- divs
    DivInt = 0x04, // -- divi

    // Memory/Stack Manipulation 0x10 - 0x25
    PushConst = 0x10,     // u16 -- pshc <literal>
    PushLocal = 0x11,     // u16 -- pshl <ident>
    StoreLocal = 0x12,    // u16 -- strl <ident>
    PushGlobal = 0x13,    // u16 -- pshg <ident>
    StoreGlobal = 0x14,   // u16 -- strg <ident>
    Pop = 0x15,           //     -- pops
    PushImmediate = 0x16, // i16 --

    // Control Flow 0x26 - 0x3F
    Jump = 0x26,               // u32 -- jump <label>
    JumpIfFalse = 0x27,        // u32 -- jmpf <label>
    JumpIfTrue = 0x28,         // u32 --
    JumpIfEqual = 0x29,        // u32 --
    JumpIfNotEqual = 0x2A,     // u32 --
    JumpIfGreaterThan = 0x2B,  // u32 --
    JumpIfLessThan = 0x2C,     // u32 --
    JumpIfGreaterEqual = 0x2D, // u32 --
    JumpIfLessEqual = 0x2E,    // u32 --

    // Comparisons and other operators 0x40
    Equal = 0x40,        // -- equl
    NotEqual = 0x41,     // -- nteq
    LessThan = 0x42,     // -- lsth
    GreaterThan = 0x43,  // -- grth
    GreaterEqual = 0x44, // -- gteq
    LessEqual = 0x45,    // -- lteq
    Not = 0x46,          // -- bnot
    LogicalAnd = 0x47,   // -- land
    LogicalOr = 0x48,    // -- lgor

    // Testing ops
    Print = 0xF5, // prnt

    // No Op
    NoOp = 0xFF,
}

impl OpCode {
    // Return a vector whose values are the sizes, in bytes, of the args the opcode accepts
    pub fn arg_sizecount(&self) -> Vec<usize> {
        match self {
            OpCode::PushConst => vec![2],
            OpCode::PushImmediate => vec![2],
            OpCode::PushGlobal => vec![2],
            _ => vec![0],
        }
    }
}

// Safe conversion from u8
impl TryFrom<u8> for OpCode {
    type Error = VMError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // Arithmetic
            0x00 => Ok(OpCode::Add),
            0x01 => Ok(OpCode::Sub),
            0x02 => Ok(OpCode::Mul),
            0x03 => Ok(OpCode::Div),
            0x04 => Ok(OpCode::DivInt),

            // Memory/Stack Manipulation
            0x10 => Ok(OpCode::PushConst),
            0x11 => Ok(OpCode::PushLocal),
            0x12 => Ok(OpCode::StoreLocal),
            0x13 => Ok(OpCode::PushGlobal),
            0x14 => Ok(OpCode::StoreGlobal),
            0x15 => Ok(OpCode::Pop),
            0x16 => Ok(OpCode::PushImmediate),

            // Control Flow
            0x26 => Ok(OpCode::Jump),
            0x27 => Ok(OpCode::JumpIfFalse),
            0x28 => Ok(OpCode::JumpIfTrue),
            0x29 => Ok(OpCode::JumpIfEqual),
            0x2A => Ok(OpCode::JumpIfNotEqual),
            0x2B => Ok(OpCode::JumpIfGreaterThan),
            0x2C => Ok(OpCode::JumpIfLessThan),
            0x2D => Ok(OpCode::JumpIfGreaterEqual),
            0x2E => Ok(OpCode::JumpIfLessEqual),

            // Comparisons
            0x40 => Ok(OpCode::Equal),
            0x41 => Ok(OpCode::NotEqual),
            0x42 => Ok(OpCode::LessThan),
            0x43 => Ok(OpCode::GreaterThan),
            0x44 => Ok(OpCode::GreaterEqual),
            0x45 => Ok(OpCode::LessEqual),
            0x46 => Ok(OpCode::Not),
            0x47 => Ok(OpCode::LogicalAnd),
            0x48 => Ok(OpCode::LogicalOr),

            // Testing
            0xF5 => Ok(OpCode::Print),
            0xFF => Ok(OpCode::NoOp),
            _ => Err(VMError::InvalidOpcode(value)),
        }
    }
}
