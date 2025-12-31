#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    NULL,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Ident(String),
}
