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

    // Memory/Stack Manipulation 0x10 - 0x2F
    PushConst = 0x10,     // u16 -- pshc <literal>
    PushLocal = 0x11,     // u16 -- pshl <ident>
    StoreLocal = 0x12,    // u16 -- strl <ident>
    PushGlobal = 0x13,    // u16 -- pshg <ident>
    StoreGlobal = 0x14,   // u16 -- strg <ident>
    Pop = 0x15,           //     -- pops
    PushImmediate = 0x16, // i16 --
    Box = 0x17,           //     -- box
    Unbox = 0x18,         //     -- unbox
    SetBox = 0x19,        //     -- setbox
    Array = 0x1A,         // u16 -- array <int>
    ArraySet = 0x1B,      //
    ArrayGet = 0x1C,      //
    ArrayPush = 0x1D,     //
    ArrayPop = 0x1E,      //
    ArrayLen = 0x1F,      //
    Map = 0x20,           // u16 -- map <int>
    MapHas = 0x21,        //
    MapKeys = 0x22,       //     -- mkys
    MapDelete = 0x23,     //
    IndexSet = 0x24,      //
    IndexGet = 0x25,      //
    Swap = 0x2D,          //     -- swap
    Dup = 0x2E,           //     -- dup

    // Control Flow 0x30 - 0x3F
    Jump = 0x30,        // u32 -- jump <label>
    JumpIfFalse = 0x31, // u32 -- jmpf <label>
    JumpIfTrue = 0x32,  // u32 --

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
    CallFromStack = 0x63,
    CallExternal = 0x64,
    PushFunctionPointer = 0x65,

    // Testing ops
    Print = 0xF5, // prnt

    // No Op
    NoOp = 0xFF,
}

impl OpCode {
    // Return a vector whose values are the sizes, in bytes, of the args the opcode accepts
    pub fn arg_sizecount(&self) -> Vec<usize> {
        match self {
            OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue => vec![4],
            OpCode::PushConst
            | OpCode::PushImmediate
            | OpCode::PushGlobal
            | OpCode::StoreGlobal
            | OpCode::Array
            | OpCode::CallFunction => vec![2],
            OpCode::StoreLocal | OpCode::PushLocal => vec![1],
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
            0x20 => Ok(OpCode::Map),
            0x21 => Ok(OpCode::MapHas),
            0x22 => Ok(OpCode::MapKeys),
            0x23 => Ok(OpCode::MapDelete),
            0x24 => Ok(OpCode::IndexSet),
            0x25 => Ok(OpCode::IndexGet),

            0x2D => Ok(OpCode::Swap),
            0x2E => Ok(OpCode::Dup),

            // Control Flow
            0x30 => Ok(OpCode::Jump),
            0x31 => Ok(OpCode::JumpIfFalse),
            0x32 => Ok(OpCode::JumpIfTrue),

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
            0x63 => Ok(OpCode::CallFromStack),
            0x64 => Ok(OpCode::CallExternal),
            0x65 => Ok(OpCode::PushFunctionPointer),

            // Testing
            0xF5 => Ok(OpCode::Print),
            0xFF => Ok(OpCode::NoOp),
            _ => Err(VMError::InvalidOpcode(value)),
        }
    }
}
pub fn op_to_arg_size(op: &str) -> usize {
    match op {
        "Jump" | "JumpIfFalse" | "JumpIfTrue" => 4,
        "PushFunctionPointer" => 3,
        "PushConst" | "PushGlobal" | "StoreGlobal" | "Array" | "CallFunction" | "Map"
        | "CallExternal" => 2,
        "StoreLocal" | "PushLocal" | "PushImmediate" => 1,
        _ => 0,
    }
}
