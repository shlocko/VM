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
    Mod = 0x05,

    // Memory/Stack Manipulation 0x10 - 0x25
    PushConst = 0x10,     // u16 -- pshc <literal>
    PushLocal = 0x11,     // u16 -- pshl <ident>
    StoreLocal = 0x12,    // u16 -- strl <ident>
    PushGlobal = 0x13,    // u16 -- pshg <ident>
    StoreGlobal = 0x14,   // u16 -- strg <ident>
    Pop = 0x15,           //     -- pops
    PushImmediate = 0x16, // i16 --
    Box = 0x17,
    Unbox = 0x18,
    SetBox = 0x19,
    Array = 0x1A,
    ArraySet = 0x1B,
    ArrayGet = 0x1C,
    ArrayPush = 0x1D,
    ArrayPop = 0x1E,
    ArrayLen = 0x1F,

    // Control Flow 0x26 - 0x3F
    Jump = 0x26,        // u32 -- jump <label>
    JumpIfFalse = 0x27, // u32 -- jmpf <label>
    JumpIfTrue = 0x28,  // u32 --

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

    // Functions
    CallFunction = 0x61, //  u16(func id) -- call <ident>
    Return = 0x62,

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
            0x17 => Ok(OpCode::Box),
            0x18 => Ok(OpCode::Unbox),
            0x19 => Ok(OpCode::SetBox),
            0x1A => Ok(OpCode::Array),
            0x1B => Ok(OpCode::ArraySet),
            0x1C => Ok(OpCode::ArrayGet),
            0x1D => Ok(OpCode::ArrayPush),
            0x1E => Ok(OpCode::ArrayPop),
            0x1F => Ok(OpCode::ArrayLen),

            // Control Flow
            0x26 => Ok(OpCode::Jump),
            0x27 => Ok(OpCode::JumpIfFalse),
            0x28 => Ok(OpCode::JumpIfTrue),

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

            // Functions
            0x61 => Ok(OpCode::CallFunction),
            0x62 => Ok(OpCode::Return),

            // Testing
            0xF5 => Ok(OpCode::Print),
            0xFF => Ok(OpCode::NoOp),
            _ => Err(VMError::InvalidOpcode(value)),
        }
    }
}

pub struct StackEffect {
    pop: u8,
    push: u8,
}

// pub fn stack_impact(op: OpCode) -> StackEffect {
//     match op {
//         OpCode::Add => StackEffect { pop: 2, push: 1 },
//         OpCode::Sub => StackEffect { pop: 2, push: 1 },
//         OpCode::Mul => StackEffect { pop: 2, push: 1 },
//         OpCode::Div => StackEffect { pop: 2, push: 1 },
//         OpCode::DivInt => StackEffect { pop: 2, push: 1 },
//         OpCode::PushConst => StackEffect { pop: 0, push: 1 },
//         OpCode::PushLocal => StackEffect { pop: 0, push: 1 },
//         OpCode::StoreLocal => StackEffect { pop: 1, push: 0 },
//         OpCode::PushGlobal => StackEffect { pop: 0, push: 1 },
//         OpCode::StoreGlobal => StackEffect { pop: 1, push: 0 },
//         OpCode::Pop => StackEffect { pop: 1, push: 0 },
//         OpCode::PushImmediate => StackEffect { pop: 0, push: 1 },
//         OpCode::Jump => StackEffect { pop: 0, push: 0 },
//         OpCode::JumpIfFalse => StackEffect { pop: 1, push: 0 },
//         OpCode::JumpIfTrue => StackEffect { pop: 1, push: 0 },
//         OpCode::Equal => StackEffect { pop: 2, push: 1 },
//         OpCode::NotEqual => StackEffect { pop: 2, push: 1 },
//         OpCode::LessThan => StackEffect { pop: 2, push: 1 },
//         OpCode::GreaterThan => StackEffect { pop: 2, push: 1 },
//         OpCode::GreaterEqual => StackEffect { pop: 2, push: 1 },
//         OpCode::LessEqual => StackEffect { pop: 2, push: 1 },
//         OpCode::Not => StackEffect { pop: 1, push: 1 },
//         OpCode::LogicalAnd => StackEffect { pop: 2, push: 1 },
//         OpCode::LogicalOr => StackEffect { pop: 2, push: 1 },
//         OpCode::Print => StackEffect { pop: 1, push: 0 },
//         OpCode::NoOp => StackEffect { pop: 0, push: 0 },
//     }
// }
