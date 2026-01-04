use std::io;

use crate::value::Value;

#[derive(Debug)]
pub enum VMError {
    // Stack Errors
    StackOverflow,
    StackUnderflow,
    InvalidStackValueType(Value, Value), // (Expected, Received)
    NotInFrame,

    // Index Errors
    InvalidLocalIndex(u16),
    InvalidGlobalIndex(u16),
    InvalidConstantIndex(u16),
    InvalidFunctionIndex(u16),

    // Opcode Errors
    InvalidOpcode(u8),
    InvalidOperandCount(u8, u8),
    InvalidOperandSize(u8, u8),

    // Operand Errors
    InvalidOperandType(Value, Value),
    InvalidUnaryOperandType(Value),

    // Arithmetic Errors
    DivisionByZero,

    //Array Errors
    IndexOutsideRangeOfArray(usize, usize), // (index, size)
    CouldNotPopArray,
}

#[derive(Debug)]
pub enum AssemblerError {
    IoError(io::Error),
    InvalidOpcode(String),
    InvalidArgument(String),
    InvalidLiteral(String),
    InvalidJumpTarget(String),
    InvalidFunctionLocation(String),
    AccessLocalOutsideFunction(String),
    InvalidFunctionEnd(String),
    InvalidFunctionCall(String),
    InvalidIdentifier(String),
    UnexpectedEof,
}

impl From<io::Error> for AssemblerError {
    fn from(error: io::Error) -> Self {
        AssemblerError::IoError(error)
    }
}

#[derive(Debug)]
pub enum JEFError {
    IoError(io::Error),
    SerdeJson(serde_json::Error),
    InvalidOpCode(String),
    InvalidArgument(String),
    DuplicateLabel(String),
    InvalidJumpTarget(String),
    CouldNotParse,
}

impl From<io::Error> for JEFError {
    fn from(error: io::Error) -> Self {
        JEFError::IoError(error)
    }
}
impl From<serde_json::Error> for JEFError {
    fn from(err: serde_json::Error) -> Self {
        JEFError::SerdeJson(err)
    }
}
