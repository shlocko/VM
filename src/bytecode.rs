use crate::{function::Function, value::Value};

pub struct Bytecode {
    pub entry: usize,
    pub consts: Vec<Value>,
    pub functions: Vec<Function>,
    pub code: Vec<u8>,
}
