use crate::value::Value;

pub struct Bytecode {
    pub consts: Vec<Value>,
    pub code: Vec<u8>,
}
