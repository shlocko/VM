use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Function {
    pub address: usize,
    pub arity: u8,
    pub locals: u8,
}
