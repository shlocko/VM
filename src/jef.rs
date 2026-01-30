use std::{collections::HashMap, fs};

use crate::{
    bytecode::Bytecode,
    error::JEFError,
    function::Function,
    opcode::OpCode,
    value::{HeapString, Value},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JEFValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JEF {
    pub consts: Vec<JEFValue>,
    pub functions: Vec<Function>,
    pub code: Vec<(String, Vec<JEFValue>)>,
}

struct FixLabel {
    offset: usize,
    label: String,
}

pub fn test_json() {
    let test_jef: JEF = JEF {
        consts: vec![JEFValue::Int(7), JEFValue::Int(8)],
        functions: vec![Function {
            address: 0,
            arity: 1,
            locals: 2,
        }],
        code: vec![
            ("pshc".to_string(), vec![JEFValue::Int(1)]),
            ("pshc".to_string(), vec![JEFValue::Int(0)]),
            ("add".to_string(), vec![]),
            ("prnt".to_string(), vec![]),
        ],
    };
    let json_text = serde_json::to_string_pretty(&test_jef).unwrap();
    println!("test_json {}", &json_text);
    let json_obj: JEF = serde_json::from_str(&json_text.as_str()).unwrap();
    println!("{:?}", json_obj);
}

pub fn assemble_json(file_name: &str) -> Result<Bytecode, JEFError> {
    let mut fix_labels: Vec<FixLabel> = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut bytecode: Bytecode = Bytecode {
        entry: 0,
        consts: Vec::new(),
        functions: Vec::new(),
        code: Vec::new(),
    };

    let file_content = fs::read_to_string(file_name)?;
    let jef: JEF = serde_json::from_str(file_content.as_str())?;

    // Convert and push const pool
    for val in jef.consts {
        let value: Value = match val {
            JEFValue::Int(v) => Value::Int(v),
            JEFValue::Float(v) => Value::Float(v),
            JEFValue::Bool(v) => Value::Bool(v),
            JEFValue::String(v) => Value::String(HeapString::new(v)),
        };
        bytecode.consts.push(value);
    }

    // Clone JEF function pool into bytecode, to be modified later with function addresses
    bytecode.functions = jef.functions.clone();

    let mut code_idx = 0;
    for code in jef.code {
        match code.0.as_str() {
            "Add" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Add as u8);
            }
            "Sub" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Sub as u8);
            }
            "Mul" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Mul as u8);
            }
            "Div" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Div as u8);
            }
            "DivInt" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::DivInt as u8);
            }
            "Mod" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Mod as u8);
            }

            // Memory/Stack Manipulation
            "PushConst" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::PushConst as u8);
                match code.1[0] {
                    JEFValue::Int(idx) => {
                        let arg = u16::to_le_bytes(idx as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "PushLocal" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::PushLocal as u8);
                match code.1[0] {
                    JEFValue::Int(idx) => {
                        let arg = u16::to_le_bytes(idx as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "StoreLocal" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::StoreLocal as u8);
                match code.1[0] {
                    JEFValue::Int(idx) => {
                        let arg = u16::to_le_bytes(idx as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "PushGlobal" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::PushGlobal as u8);
                match code.1[0] {
                    JEFValue::Int(idx) => {
                        let arg = u16::to_le_bytes(idx as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "StoreGlobal" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::StoreGlobal as u8);
                match code.1[0] {
                    JEFValue::Int(idx) => {
                        let arg = u16::to_le_bytes(idx as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "Pop" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Pop as u8);
            }
            "PushImmediate" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::PushImmediate as u8);
                match code.1[0] {
                    JEFValue::Int(val) => {
                        let arg = i16::to_le_bytes(val as i16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "Box" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Box as u8);
            }
            "Unbox" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Unbox as u8);
            }
            "SetBox" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::SetBox as u8);
            }
            "Array" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::Array as u8);
                match code.1[0] {
                    JEFValue::Int(val) => {
                        let arg = u16::to_le_bytes(val as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "ArraySet" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::ArraySet as u8);
            }
            "ArrayGet" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::ArrayGet as u8);
            }
            "ArrayPush" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::ArrayPush as u8);
            }
            "ArrayPop" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::ArrayPop as u8);
            }
            "ArrayLen" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::ArrayLen as u8);
            }

            // Control Flow
            "Jump" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::Jump as u8);
                match code.1[0].clone() {
                    JEFValue::String(label) => {
                        if let Some(target) = labels.get(&label) {
                            let location = u32::to_le_bytes(*target);
                            bytecode.code.push(location[0]);
                            bytecode.code.push(location[1]);
                            bytecode.code.push(location[2]);
                            bytecode.code.push(location[3]);
                        } else {
                            fix_labels.push(FixLabel {
                                offset: bytecode.code.len(),
                                label: label,
                            });

                            let location = u32::to_le_bytes(0);
                            bytecode.code.push(location[0]);
                            bytecode.code.push(location[1]);
                            bytecode.code.push(location[2]);
                            bytecode.code.push(location[3]);
                        }
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected name for label at position: {}",
                            code_idx
                        )));
                    }
                }
            }
            "JumpIfFalse" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::JumpIfFalse as u8);
                match code.1[0].clone() {
                    JEFValue::String(label) => {
                        if let Some(target) = labels.get(&label) {
                            let location = u32::to_le_bytes(*target);
                            bytecode.code.push(location[0]);
                            bytecode.code.push(location[1]);
                            bytecode.code.push(location[2]);
                            bytecode.code.push(location[3]);
                        } else {
                            fix_labels.push(FixLabel {
                                offset: bytecode.code.len(),
                                label: label,
                            });

                            let location = u32::to_le_bytes(0);
                            bytecode.code.push(location[0]);
                            bytecode.code.push(location[1]);
                            bytecode.code.push(location[2]);
                            bytecode.code.push(location[3]);
                        }
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected name for label at position: {}",
                            code_idx
                        )));
                    }
                }
            }
            "JumpIfTrue" => {
                check_arg_count(&code, 1, code_idx)?;
                bytecode.code.push(OpCode::JumpIfTrue as u8);
                match code.1[0].clone() {
                    JEFValue::String(label) => {
                        if let Some(target) = labels.get(&label) {
                            let location = u32::to_le_bytes(*target);
                            bytecode.code.push(location[0]);
                            bytecode.code.push(location[1]);
                            bytecode.code.push(location[2]);
                            bytecode.code.push(location[3]);
                        } else {
                            fix_labels.push(FixLabel {
                                offset: bytecode.code.len(),
                                label: label,
                            });

                            let location = u32::to_le_bytes(0);
                            bytecode.code.push(location[0]);
                            bytecode.code.push(location[1]);
                            bytecode.code.push(location[2]);
                            bytecode.code.push(location[3]);
                        }
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected name for label at position: {}",
                            code_idx
                        )));
                    }
                }
            }

            // Comparisons and other operators
            "Equal" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Equal as u8);
            }
            "NotEqual" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::NotEqual as u8);
            }
            "LessThan" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::LessThan as u8);
            }
            "GreaterThan" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::GreaterThan as u8);
            }
            "GreaterEqual" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::GreaterEqual as u8);
            }
            "LessEqual" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::LessEqual as u8);
            }
            "Not" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Not as u8);
            }
            "LogicalAnd" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::LogicalAnd as u8);
            }
            "LogicalOr" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::LogicalOr as u8);
            }

            // Functions
            "CallFunction" => {
                check_arg_count(&code, 1, code_idx)?;
                match code.1[0] {
                    JEFValue::Int(idx) => {
                        let location = bytecode.code.len();
                        bytecode.functions[idx as usize].address = location;
                        bytecode.code.push(OpCode::CallFunction as u8);
                        let arg = u16::to_le_bytes(idx as u16);
                        bytecode.code.push(arg[0]);
                        bytecode.code.push(arg[1]);
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected 16bit Integer at position: {}",
                            code_idx
                        )))
                    }
                }
            }
            "Return" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Return as u8);
            }

            // Testing ops?
            "Print" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::Print as u8);
            }

            // No Op
            "NoOp" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.code.push(OpCode::NoOp as u8);
            }
            "Main" => {
                check_arg_count(&code, 0, code_idx)?;
                bytecode.entry = bytecode.code.len();
                bytecode.code.push(OpCode::NoOp as u8);
            }
            "Label" => {
                check_arg_count(&code, 1, code_idx)?;
                match code.1[0].clone() {
                    JEFValue::String(label) => {
                        if let Some(_) = labels.get(&label) {
                            return Err(JEFError::DuplicateLabel(format!(
                                "Duplicate label: {}, found at position: {}",
                                &label, code_idx
                            )));
                        } else {
                            let location = bytecode.code.len();
                            labels.insert(label, location as u32);
                        }
                    }
                    _ => {
                        return Err(JEFError::InvalidArgument(format!(
                            "Expected name for label at position: {}",
                            code_idx
                        )));
                    }
                }
                bytecode.code.push(OpCode::NoOp as u8);
            }
            _ => {
                return Err(JEFError::InvalidOpCode(format!(
                    "Invalid opcode: {}",
                    code.0
                )));
            }
        }
        code_idx += 1;
    }

    for label in fix_labels {
        if let Some(loc) = labels.get(&label.label) {
            let bytes = u32::to_le_bytes(*loc);
            bytecode.code[label.offset] = bytes[0];
            bytecode.code[label.offset + 1] = bytes[1];
            bytecode.code[label.offset + 2] = bytes[2];
            bytecode.code[label.offset + 3] = bytes[3];
        } else {
            return Err(JEFError::InvalidJumpTarget(format!(
                "Invalid jump target: {}",
                label.label
            )));
        }
    }
    return Ok(bytecode);
}

fn check_arg_count(
    code: &(String, Vec<JEFValue>),
    expected_arg: usize,
    code_idx: usize,
) -> Result<(), JEFError> {
    if code.1.len() != expected_arg {
        return Err(JEFError::InvalidArgument(format!(
            "Expected {} arguments for opcode: {}, at location: {}",
            expected_arg, code.0, code_idx
        )));
    }
    Ok(())
}
