use std::io;

use crate::value::Value;

#[derive(Debug)]
pub enum VMError {
    // Stack Errors
    StackOverflow,
    StackUnderflow,
    InvalidStackValueType(Value, Value), // (Expected, Received)

    // Index Errors
    InvalidLocalIndex(u16),
    InvalidGlobalIndex(u16),
    InvalidConstantIndex(u16),
    InvalidFunctionIndex(u16),

    // Opcode Errors
    InvalidOpcode(u8),
    InvalidOperandCount(u8, u8),
    InvalidOperandSize(u8, u8),

    // Operand erros
    InvalidOperandType(Value, Value),
    InvalidUnaryOperandType(Value),
}

#[derive(Debug)]
pub enum AssemblerError {
    IoError(io::Error),
    InvalidOpcode(String),
    InvalidArgument(String),
    InvalidLiteral(String),
    InvalidJumpTarget(String),
    UnexpectedEof,
}

impl From<io::Error> for AssemblerError {
    fn from(error: io::Error) -> Self {
        AssemblerError::IoError(error)
    }
}
