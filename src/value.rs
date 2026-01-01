use std::{collections::HashMap, rc::Rc};

pub type HeapString = Rc<String>;
pub type HeapVec = Rc<Vec<Value>>;
pub type HeapMap = Rc<HashMap<String, Value>>;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    NULL,
    Int(i64),
    Float(f64),
    String(HeapString),
    Bool(bool),
    Ident(String),
}
