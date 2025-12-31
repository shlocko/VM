use crate::bytecode::Bytecode;
use crate::error::VMError;
use crate::memory::Stack;
use crate::opcode::OpCode;
use crate::value::Value;

pub struct VM {
    stack: Stack,
    consts: Vec<Value>,
    globals: Vec<Value>,
    code: Vec<u8>,
    ip: usize,
}

impl VM {
    fn u32_from_le(&mut self) -> u32 {
        let val = u32::from_le_bytes([
            self.code[self.ip + 1],
            self.code[self.ip + 2],
            self.code[self.ip + 3],
            self.code[self.ip + 4],
        ]);
        self.ip += 4;
        return val;
    }
    fn u16_from_le(&mut self) -> u16 {
        let val = u16::from_le_bytes([self.code[self.ip + 1], self.code[self.ip + 2]]);
        self.ip += 2;
        return val;
    }
    fn i16_from_le(&mut self) -> i16 {
        let val = i16::from_le_bytes([self.code[self.ip + 1], self.code[self.ip + 2]]);
        self.ip += 2;
        return val;
    }
    pub fn new(init_stack_cap: usize) -> Self {
        Self {
            stack: Stack::new(init_stack_cap, usize::MAX),
            consts: Vec::new(),
            globals: Vec::new(),
            code: Vec::new(),
            ip: 0,
        }
    }
    pub fn load_code(&mut self, bytecode: Bytecode) {
        self.code = bytecode.code;
        self.consts = bytecode.consts;
    }
    pub fn execute(&mut self) -> Result<(), VMError> {
        loop {
            let opcode = OpCode::try_from(self.code[self.ip])?;
            // println!("Stack before {:?}: {:?}", opcode, self.stack);
            match opcode {
                // Arithmetic
                OpCode::Add => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    // println!("stack after pops: {:?}", self.stack);
                    match (lop, rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            let result = l + r;
                            self.stack.push(Value::Int(result))?;
                            // println!("add: {} + {} = {}", l, r, result);
                        }
                        _ => {
                            println!("Wrong values");
                        }
                    }
                }
                OpCode::Sub => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    // println!("stack after pops: {:?}", self.stack);
                    match (lop, rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            let result = l - r;
                            self.stack.push(Value::Int(result))?;
                            // println!("add: {} + {} = {}", l, r, result);
                        }
                        _ => {
                            println!("Wrong values");
                        }
                    }
                }

                // Memory/Stack Manipulation
                OpCode::PushConst => {
                    let val = self.u16_from_le();
                    self.stack.push(self.consts[val as usize].clone())?;
                }
                OpCode::PushImmediate => {
                    let val = self.u16_from_le();

                    self.stack.push(Value::Int(val as i64))?;
                }
                OpCode::StoreGlobal => {
                    let val = self.stack.pop()?;
                    let arg = self.u16_from_le();
                    if arg == self.globals.len() as u16 {
                        self.globals.push(val);
                    } else {
                        self.globals[arg as usize] = val;
                    }
                }
                OpCode::PushGlobal => {
                    let arg = self.u16_from_le();
                    if arg < self.globals.len() as u16 {
                        self.stack.push(self.globals[arg as usize].clone())?;
                    }
                }
                OpCode::Pop => {
                    self.stack.pop()?;
                }

                // Control Flow
                OpCode::Jump => {
                    let arg = self.u32_from_le() as usize;
                    // println!("JUMP {}", arg);
                    self.ip = arg;
                }
                OpCode::JumpIfFalse => {
                    let val = self.stack.pop()?;
                    let arg = self.u32_from_le() as usize;
                    match val {
                        Value::Bool(v) => {
                            if !v {
                                self.ip = arg;
                            }
                        }
                        _ => return Err(VMError::InvalidStackValueType(Value::Bool(true), val)),
                    }
                }
                OpCode::JumpIfTrue => {
                    let val = self.stack.pop()?;
                    let arg = self.u32_from_le() as usize;
                    match val {
                        Value::Bool(true) => {
                            self.ip = arg;
                        }
                        Value::Bool(false) => {}
                        _ => return Err(VMError::InvalidStackValueType(Value::Bool(true), val)),
                    }
                }

                // Comparison and other operators
                OpCode::Equal => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            if l == r {
                                result = true;
                            }
                        }
                        (Value::Bool(l), Value::Bool(r)) => {
                            if l == r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::NotEqual => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            if l != r {
                                result = true;
                            }
                        }
                        (Value::Bool(l), Value::Bool(r)) => {
                            if l != r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::LessThan => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            if l < r {
                                result = true;
                            }
                        }
                        (Value::Float(l), Value::Float(r)) => {
                            if l < r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::GreaterThan => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            if l > r {
                                result = true;
                            }
                        }
                        (Value::Float(l), Value::Float(r)) => {
                            if l > r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::GreaterEqual => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            if l >= r {
                                result = true;
                            }
                        }
                        (Value::Float(l), Value::Float(r)) => {
                            if l >= r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::LessEqual => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Int(l), Value::Int(r)) => {
                            if l <= r {
                                result = true;
                            }
                        }
                        (Value::Float(l), Value::Float(r)) => {
                            if l <= r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::Not => {
                    let val = self.stack.pop()?;
                    match &val {
                        Value::Bool(v) => {
                            self.stack.push(Value::Bool(!v))?;
                        }
                        _ => return Err(VMError::InvalidUnaryOperandType(val)),
                    }
                }
                OpCode::LogicalAnd => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Bool(l), Value::Bool(r)) => {
                            if *l && *r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }
                OpCode::LogicalOr => {
                    let rop = self.stack.pop()?;
                    let lop = self.stack.pop()?;
                    let mut result: bool = false;
                    match (&lop, &rop) {
                        (Value::Bool(l), Value::Bool(r)) => {
                            if *l || *r {
                                result = true;
                            }
                        }
                        _ => return Err(VMError::InvalidOperandType(lop, rop)),
                    }
                    self.stack.push(Value::Bool(result))?;
                }

                // Testing ops
                OpCode::Print => {
                    let val = self.stack.pop()?;
                    println!("{:?}", val);
                }

                // No Op
                OpCode::NoOp => {}
                _ => {
                    panic!("Invalid opcode")
                }
            }
            // println!("Stack after {:?}: {:?}", opcode, self.stack);
            self.ip += 1;
            if self.ip >= self.code.len() {
                break;
            }
        }
        Ok(())
    }
}
