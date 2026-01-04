use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Function {
    pub address: usize,
    pub arity: u8,
    pub locals: u8,
}
