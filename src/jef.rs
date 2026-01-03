use crate::{function::Function, value::Value};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum JEFValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Deserialize)]
pub struct JEF {
    pub consts: Vec<JEFValue>,
    pub functions: Vec<Function>,
    pub code: Vec<(String, Vec<JEFValue>)>,
}
