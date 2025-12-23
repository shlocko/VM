use crate::value::Value;

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
    pub fn push(&mut self, val: Value) {
        self.values.push(val);
    }
    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }
}
