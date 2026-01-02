use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::VMError;

pub type HeapString = Rc<String>;
pub type HeapVec = Rc<RefCell<Vec<Value>>>;
pub type HeapMap = Rc<RefCell<HashMap<String, Value>>>;
pub type HeapValue = Rc<RefCell<Value>>;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    NULL,
    Int(i64),
    Float(f64),
    String(HeapString),
    Bool(bool),
    Ident(String),
    HeapValue(HeapValue),
    Function(usize),
    Array(HeapVec),
}

impl Value {
    pub fn new_box(val: Value) -> Value {
        return Value::HeapValue(Rc::new(RefCell::new(val.clone())));
    }
    pub fn new_array(vals: Vec<Value>) -> Value {
        return Value::Array(Rc::new(RefCell::new(vals)));
    }
    pub fn set_to_array(idx: usize, val: Value, arr: Value) -> Result<(), VMError> {
        match arr {
            Value::Array(boxed_array) => {
                let mut unboxed = boxed_array.borrow_mut();
                if idx >= unboxed.len() {
                    return Err(VMError::IndexOutsideRangeOfArray(idx, unboxed.len()));
                }
                unboxed[idx] = val;
            }
            _ => return Err(VMError::InvalidUnaryOperandType(val)),
        }

        Ok(())
    }
    pub fn get_from_array(idx: usize, arr: Value) -> Result<Value, VMError> {
        match arr {
            Value::Array(boxed_array) => {
                let unboxed = boxed_array.borrow();
                if idx >= unboxed.len() {
                    return Err(VMError::IndexOutsideRangeOfArray(idx, unboxed.len()));
                }
                return Ok(unboxed[idx].clone());
            }
            _ => return Err(VMError::InvalidUnaryOperandType(arr)),
        }
    }
    pub fn push_to_array(val: Value, arr: Value) -> Result<(), VMError> {
        match arr {
            Value::Array(boxed_array) => {
                let mut unboxed = boxed_array.borrow_mut();
                unboxed.push(val);
                Ok(())
            }
            _ => return Err(VMError::InvalidUnaryOperandType(arr)),
        }
    }
    pub fn pop_from_array(arr: Value) -> Result<Value, VMError> {
        match arr {
            Value::Array(boxed_array) => {
                let mut unboxed = boxed_array.borrow_mut();
                if let Some(v) = unboxed.pop() {
                    Ok(v)
                } else {
                    Err(VMError::CouldNotPopArray)
                }
            }
            _ => return Err(VMError::InvalidUnaryOperandType(arr)),
        }
    }
    pub fn array_len(arr: Value) -> Result<Value, VMError> {
        match arr {
            Value::Array(boxed_array) => {
                let unboxed = boxed_array.borrow();
                return Ok(Value::Int(unboxed.len() as i64));
            }
            _ => return Err(VMError::InvalidUnaryOperandType(arr)),
        }
    }
}
