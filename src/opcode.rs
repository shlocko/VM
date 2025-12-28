#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    // Arithmetic
    AddInt = 0x00,   // addi
    AddFloat = 0x01, // addf
    SubInt = 0x02,   // subi
    SubFloat = 0x03, // subf
    MulInt = 0x04,   // muli
    MulFloat = 0x05, // mulf
    DivInt = 0x06,   // divi
    DivFloat = 0x07, // divf

    // Memory/Stack Manipulation
    PushConst = 0x10,     // pshc u16
    PushLocal = 0x11,     // pshl u16
    CreateLocal = 0x12,   // crtl u16
    StoreLocal = 0x13,    // strl u16
    PushGlobal = 0x14,    // pshg u16
    CreateGlobal = 0x15,  // crtg u16
    StoreGlobal = 0x16,   // strg u16
    Pop = 0x17,           // pops
    PushImmediate = 0x18, // pshm i16

    // Testing ops
    Print = 0xF5, // prnt
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
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // Arithmetic
            0x00 => Ok(OpCode::AddInt),
            0x01 => Ok(OpCode::AddFloat),
            0x02 => Ok(OpCode::SubInt),
            0x03 => Ok(OpCode::SubFloat),
            0x04 => Ok(OpCode::MulInt),
            0x05 => Ok(OpCode::MulFloat),
            0x06 => Ok(OpCode::DivInt),
            0x07 => Ok(OpCode::DivFloat),

            // Memory/Stack Manipulation
            0x10 => Ok(OpCode::PushConst),
            0x11 => Ok(OpCode::PushLocal),
            0x12 => Ok(OpCode::CreateLocal),
            0x13 => Ok(OpCode::StoreLocal),
            0x14 => Ok(OpCode::PushGlobal),
            0x15 => Ok(OpCode::CreateGlobal),
            0x16 => Ok(OpCode::StoreGlobal),
            0x17 => Ok(OpCode::Pop),
            0x18 => Ok(OpCode::PushImmediate),

            // Testing
            0xF5 => Ok(OpCode::Print),
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}
