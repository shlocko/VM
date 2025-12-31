use crate::{error::VMError, value::Value};

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
    max_size: usize,
    pointer: usize,
}

impl Stack {
    pub fn new(init_capacity: usize, max_size: usize) -> Self {
        let mut stack = Self {
            values: Vec::new(),
            max_size,
            pointer: 0,
        };

        stack.values.resize(init_capacity, Value::default());

        return stack;
    }
    pub fn push(&mut self, val: Value) -> Result<(), VMError> {
        if self.pointer < self.max_size {
            self.values[self.pointer] = val;
            self.pointer += 1;
            if self.pointer >= self.values.len() {
                self.values.resize(self.values.len(), Value::default());
            }
            Ok(())
        } else {
            Err(VMError::StackOverflow)
        }
    }
    pub fn pop(&mut self) -> Result<Value, VMError> {
        if self.pointer <= 0 {
            return Err(VMError::StackUnderflow);
        }
        self.pointer -= 1;
        return Ok(self.values[self.pointer].clone());
    }
}
