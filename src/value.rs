use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::VMError, jef::JEFValue};

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
    Map(HeapMap),
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

    pub fn new_map(map: HashMap<String, Value>) -> Value {
        return Value::Map(Rc::new(RefCell::new(map)));
    }

    pub fn map_has(map: Value, key: Value) -> bool {
        match (map, key) {
            (Value::Map(map), Value::String(s)) => {
                if map.borrow().contains_key(s.as_ref()) {
                    return true;
                } else {
                    return false;
                }
            }
            _ => {
                todo!();
            }
        }
    }

    pub fn map_keys(map: Value) -> Result<Vec<Value>, VMError> {
        match map {
            Value::Map(map) => {
                let keys: Vec<Value> = map
                    .borrow()
                    .keys()
                    .map(|k| Value::String(Rc::new((*k).clone())))
                    .collect();
                return Ok(keys);
            }
            _ => Err(VMError::ExpectedMap),
        }
    }

    pub fn to_jef(self) -> JEFValue {
        match self {
            Value::Int(v) => JEFValue::Int(v),
            Value::Float(v) => JEFValue::Float(v),
            Value::Bool(v) => JEFValue::Bool(v),
            Value::String(v) => JEFValue::String((*v).clone()),
            _ => panic!(),
        }
    }
}
