use crate::bytecode::Bytecode;
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
    pub fn new(max_stack_size: usize) -> Self {
        Self {
            stack: Stack::new(max_stack_size),
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
    pub fn execute(&mut self) -> Result<bool, String> {
        loop {
            let opcode = OpCode::try_from(self.code[self.ip])?;
            // println!("Test, {}, {:?}, {:?}", self.ip, self.stack, opcode);
            println!("Stack before {:?}: {:?}", opcode, self.stack);
            match opcode {
                // Arithmetic
                OpCode::AddInt => {
                    let lop = self.stack.pop();
                    let rop = self.stack.pop();
                    println!("stack after pops: {:?}", self.stack);
                    match (lop, rop) {
                        (Some(Value::Int(l)), Some(Value::Int(r))) => {
                            let result = l + r;
                            self.stack.push(Value::Int(result));
                            println!("add: {} + {} = {}", l, r, result);
                        }
                        _ => {
                            println!("Wrong values");
                        }
                    }
                }

                // Memory/Stack Manipulation
                OpCode::PushConst => {
                    // Read arg from bytes encoded as 2 LE bytes
                    let val = u16::from_le_bytes([self.code[self.ip + 1], self.code[self.ip + 2]]);
                    // Push const at location indicated by arg to stack
                    self.stack.push(self.consts[val as usize].clone());

                    // Increment IP by the number of bytes in opcode
                    for num in opcode.arg_sizecount() {
                        self.ip += num;
                    }
                }
                OpCode::PushImmediate => {
                    let val = u16::from_le_bytes([self.code[self.ip + 1], self.code[self.ip + 2]]);

                    self.stack.push(Value::Int(val as i64));
                    self.ip += 2;
                }
                OpCode::PushGlobal => {
                    let global_idx =
                        u16::from_le_bytes([self.code[self.ip + 1], self.code[self.ip + 2]]);
                    self.stack.push(self.globals[global_idx as usize].clone());
                    self.ip += 2;
                }
                OpCode::StoreGlobal => {
                    let val = self.stack.pop();
                }
                OpCode::Print => {
                    if let Some(val) = self.stack.pop() {
                        println!("{:?}", val);
                    };
                }
                _ => {
                    panic!("Invalid opcode")
                }
            }
            println!("Stack after {:?}: {:?}", opcode, self.stack);
            self.ip += 1;
            if self.ip >= self.code.len() {
                break;
            }
        }
        Ok(true)
    }
}
