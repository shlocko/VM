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
    PushLocal = 0x11,
    StoreLocal = 0x12,
    PushGlobal = 0x13,
    StoreGlobal = 0x14,
    Pop = 0x15,
}

impl OpCode {
    // Return a vector whose values are the sizes, in bytes, of the args the opcode accepts
    pub fn arg_sizecount(&self) -> Vec<usize> {
        match self {
            OpCode::PushConst => vec![2],
            _ => vec![0],
        }
    }
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
            0x11 => Ok(OpCode::PushLocal),
            0x12 => Ok(OpCode::StoreLocal),
            0x13 => Ok(OpCode::PushGlobal),
            0x14 => Ok(OpCode::StoreGlobal),
            0x15 => Ok(OpCode::Pop),
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}
