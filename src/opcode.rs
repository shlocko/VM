#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    // Arithmetic
    Add = 0x00,
    Sub = 0x01,
    Mul = 0x02,
    Div = 0x03,
    IDiv = 0x04,

    // Memory/Stack Manipulation
    PushConst = 0x10,
    PushVar = 0x11,
    StoreVar = 0x12,
    Pop = 0x13,
}

// Safe conversion from u8
impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // Arithmetic
            0x00 => Ok(OpCode::Add),
            0x01 => Ok(OpCode::Sub),
            0x02 => Ok(OpCode::Mul),
            0x03 => Ok(OpCode::Div),
            0x04 => Ok(OpCode::IDiv),

            // Memory/Stack Manipulation
            0x10 => Ok(OpCode::PushConst),
            0x11 => Ok(OpCode::PushVar),
            0x12 => Ok(OpCode::StoreVar),
            0x13 => Ok(OpCode::Pop),
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}
