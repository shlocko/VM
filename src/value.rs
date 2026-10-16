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
    Char(char),
    Bool(bool),
    Ident(String),
    HeapValue(HeapValue),
    Function(usize, Vec<Value>),
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

    pub fn map_delete(map: Value, key: Value) -> Result<bool, VMError> {
        match map {
            Value::Map(map) => match key {
                Value::String(key) => {
                    let val = map.borrow_mut().remove(key.as_str());
                    match val {
                        Some(_) => {
                            return Ok(true);
                        }
                        None => return Ok(false),
                    }
                }
                _ => {
                    return Err(VMError::InvalidStackValueType(
                        Value::String(Rc::new(String::new())),
                        key,
                    ))
                }
            },
            _ => {
                return Err(VMError::InvalidStackValueType(
                    Value::Map(Rc::new(RefCell::new(HashMap::new()))),
                    map,
                ))
            }
        }
    }

    pub fn index_set(data: Value, key: Value, val: Value) -> Result<bool, VMError> {
        match data {
            Value::Map(map) => match key {
                Value::String(key) => {
                    let res = map.borrow_mut().insert((*key).clone(), val);
                    match res {
                        Some(_) => Ok(true),
                        None => Ok(false),
                    }
                }
                _ => {
                    return Err(VMError::InvalidStackValueType(
                        Value::String(Rc::new(String::new())),
                        key,
                    ))
                }
            },
            Value::Array(arr) => match key {
                Value::Int(idx) => {
                    arr.borrow_mut().insert(idx as usize, val);
                    Ok(true)
                }
                _ => return Err(VMError::InvalidStackValueType(Value::Int(0), key)),
            },
            _ => todo!("Bad type for index_set not implemented"),
        }
    }
    pub fn index_get(data: Value, key: Value) -> Result<Value, VMError> {
        match data {
            Value::Map(map) => match key {
                Value::String(key) => {
                    let map = map.borrow();
                    let res = map.get((*key).as_str());
                    match res {
                        Some(val) => Ok((*val).clone()),
                        None => Ok(Value::NULL),
                    }
                }
                _ => {
                    return Err(VMError::InvalidStackValueType(
                        Value::String(Rc::new(String::new())),
                        key,
                    ))
                }
            },
            Value::Array(arr) => match key {
                Value::Int(idx) => {
                    let arr = arr.borrow();
                    let res = arr.get(idx as usize);
                    match res {
                        Some(val) => Ok((*val).clone()),
                        None => Ok(Value::NULL),
                    }
                }
                _ => return Err(VMError::InvalidStackValueType(Value::Int(0), key)),
            },
            _ => todo!("Bad type for index_get not implemented"),
        }
    }

    pub fn to_jef(self) -> JEFValue {
        match self {
            Value::Int(v) => JEFValue::Int(v),
            Value::Float(v) => JEFValue::Float(v),
            Value::Bool(v) => JEFValue::Bool(v),
            Value::String(v) => JEFValue::String((*v).clone()),
            Value::Ident(v) => JEFValue::String(v.to_string()),
            Value::NULL => JEFValue::NULL,
            _ => panic!("{:?}", self),
        }
    }
}
