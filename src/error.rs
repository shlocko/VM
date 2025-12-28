use std::io;

pub enum VMErrorKind {
    // Stack Errors
    StackOverflow,
    StackUnderflow,

    // Index Errors
    InvalidLocalIndex(u16),
    InvalidGlobalIndex(u16),
    InvalidConstantIndex(u16),
    InvalidFunctionIndex(u16),

    // Opcode Errors
    InvalidOpcode(u8),
    InvalidOperandCount(u8, u8),
    InvalidOperandSize(u8, u8),
}

pub enum AssemblerErrorKind {
    IoError(io::Error),
    InvalidOpcode(String),
    InvalidArgument(String),
    UnexpectedEof,
}

// Automatically convert io::Error to AssemblerError
impl From<io::Error> for AssemblerErrorKind {
    fn from(error: io::Error) -> Self {
        AssemblerErrorKind::IoError(error)
    }
}
