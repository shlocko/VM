use crate::{error::VMError, value::Value};

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
    max_size: usize,
    pointer: usize,
    pub frames: Vec<StackFrame>,
    frame_pointer: usize,
}

impl Stack {
    pub fn new(init_capacity: usize, max_size: usize) -> Self {
        let mut stack = Self {
            values: Vec::new(),
            max_size,
            pointer: 0,
            frames: Vec::new(),
            frame_pointer: 0,
        };

        stack.values.resize(init_capacity, Value::default());

        return stack;
    }
    pub fn push(&mut self, val: Value) -> Result<(), VMError> {
        if self.pointer < self.max_size {
            if self.pointer >= self.values.len() {
                self.values.resize(self.values.len() * 2, Value::default());
            }
            self.values[self.pointer] = val;
            self.pointer += 1;
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
        // return Ok(self.values[self.pointer].clone());
        return Ok(std::mem::replace(
            &mut self.values[self.pointer],
            Value::default(),
        ));
    }
    pub fn push_frame(
        &mut self,
        args: Vec<Value>,
        locals: usize,
        return_address: usize,
    ) -> Result<(), VMError> {
        let ptr = self.pointer;
        let frame = StackFrame {
            return_address,
            previous_frame_pointer: ptr,
            locals,
        };
        for arg in args {
            self.push(arg)?;
        }
        self.frames.push(frame);
        self.pointer += locals;
        if self.pointer >= self.values.len() {
            self.values.resize(self.values.len() * 2, Value::default());
        }
        return Ok(());
    }
    pub fn pop_frame(&mut self) -> Result<usize, VMError> {
        if let Some(frame) = self.frames.pop() {
            self.pointer = frame.previous_frame_pointer;

            return Ok(frame.return_address);
        } else {
            return Err(VMError::StackUnderflow);
        }
    }
    pub fn peek_local(&mut self, idx: u8) -> Result<Value, VMError> {
        if let Some(frame_ptr) = self.frames.last() {
            let val = self.values[frame_ptr.previous_frame_pointer + idx as usize].clone();

            return Ok(val);
        } else {
            return Err(VMError::NotInFrame);
        }
    }
    pub fn set_local(&mut self, val: Value, idx: u8) {
        if let Some(frame_ptr) = self.frames.last() {
            self.values[frame_ptr.previous_frame_pointer + idx as usize] = val;
        }
    }
}

#[derive(Debug)]
pub struct StackFrame {
    return_address: usize,
    previous_frame_pointer: usize,
    locals: usize,
}
