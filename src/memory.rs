use crate::{error::VMError, value::Value};

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
    max_size: usize,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Self {
            values: Vec::with_capacity(max_size),
            max_size,
        }
    }
    pub fn push(&mut self, val: Value) -> Result<(), VMError> {
        if self.values.len() >= self.max_size {
            return Err(VMError::StackOverflow);
        }
        self.values.push(val);
        Ok(())
    }
    pub fn pop(&mut self) -> Result<Value, VMError> {
        let result = self.values.pop();
        match result {
            Some(val) => {
                return Ok(val);
            }
            None => {
                return Err(VMError::StackUnderflow);
            }
        }
    }
}
