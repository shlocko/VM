use crate::error::AssemblerError;
use crate::function::Function;
use crate::jef::{new_jef, JEFValue, JEF};
use crate::opcode::OpCode;
use crate::value::{HeapString, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct FixLabel {
    offset: usize,
    label: String,
}

struct CurFunc {
    name: String,
    locals: HashMap<String, u8>,
    done: bool,
}

pub fn assemble() -> Result<JEF, AssemblerError> {
    let file = File::open("program.fasm")?;
    let reader = BufReader::new(file);
    let mut linenum = 0;
    // Vectors for binary format
    let mut bin_vec: Vec<(String, Vec<JEFValue>)> = Vec::new();
    let mut consts: Vec<JEFValue> = Vec::new();
    let mut functions: Vec<Function> = Vec::new();

    let mut globals_names: HashMap<String, u16> = HashMap::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    // let mut fix_labels: Vec<FixLabel> = Vec::new();
    let mut func_names: HashMap<String, usize> = HashMap::new();
    let mut entry = 0;
    let mut current_function: CurFunc = CurFunc {
        name: "".to_string(),
        locals: HashMap::new(),
        done: true,
    };

    for line in reader.lines() {
        linenum += 1;
        let line = line?;
        let data: Vec<&str> = line.split(" ").collect();
        let op = data[0];
        match op {
            "main" => bin_vec.push(("Main".to_string(), vec![])),
            "#" => {}
            "" => {}
            "add" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Add", &[]));
            }
            "sub" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Sub", &[]));
            }
            "mul" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Mul", &[]));
            }
            "div" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                // bin_vec.push(OpCode::Div as u8);
                bin_vec.push(new_jef("Div", &[]));
            }
            "divi" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("DivInt", &[]));
            }
            "pshc" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum);
                let idx: u16;
                match arg {
                    Ok(val) => {
                        let index = consts.iter().position(|x| *x == val.clone().to_jef());
                        match index {
                            Some(val) => {
                                idx = val as u16;
                            }
                            None => {
                                consts.push(val.clone().to_jef());
                                idx = (consts.len() - 1) as u16;
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
                bin_vec.push(new_jef("PushConst", &[JEFValue::Int(idx.into())]))
            }
            "pshl" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }

                if current_function.done {
                    return Err(AssemblerError::AccessLocalOutsideFunction(format!(
                        "Attempeted to store local outside function on line: {}",
                        linenum
                    )));
                }
                let val = parse_literal(data[1], linenum)?;
                match val {
                    Value::Ident(ident) => {
                        if let Some(idx) = current_function.locals.get(&ident) {
                            bin_vec.push(new_jef("PushLocal", &[JEFValue::Int((*idx).into())]))
                        } else {
                            return Err(AssemblerError::InvalidIdentifier(format!(
                                "Access local that isn't defined at line: {}",
                                linenum
                            )));
                        }
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected identifier on line: {}",
                            linenum
                        )));
                    }
                }
            }
            "strl" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                if current_function.done {
                    return Err(AssemblerError::AccessLocalOutsideFunction(format!(
                        "Attempeted to store local outside function on line: {}",
                        linenum
                    )));
                }
                let val = parse_literal(data[1], linenum)?;
                match val {
                    Value::Ident(ident) => {
                        if let Some(idx) = current_function.locals.get(&ident) {
                            bin_vec.push(new_jef("StoreLocal", &[JEFValue::Int((*idx).into())]));
                        } else {
                            // println!("storelocal {}", ident);
                            let idx = current_function.locals.len();
                            current_function.locals.insert(ident, idx as u8);
                            bin_vec.push(new_jef("StoreLocal", &[JEFValue::Int(idx as i64)]))
                        }
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected identifier on line: {}",
                            linenum
                        )));
                    }
                }
            }
            "strg" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum);
                let idx: u16;
                match arg {
                    Ok(Value::Ident(name)) => {
                        if let Some(id) = globals_names.get(name.as_str()) {
                            idx = *id;
                        } else {
                            let id = globals_names.iter().count() as u16;
                            let _ = globals_names.insert(name, id);
                            idx = id;
                        }
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected identifier at line: {}",
                            linenum
                        )));
                    }
                }
                bin_vec.push(new_jef("StoreGlobal", &[JEFValue::Int(idx.into())]))
            }
            "pshg" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum);
                let idx: u16;
                match arg {
                    Ok(Value::Ident(name)) => {
                        if let Some(id) = globals_names.get(name.as_str()) {
                            idx = *id;
                        } else {
                            return Err(AssemblerError::InvalidArgument(format!(
                                "Expected global identifier at line: {}",
                                linenum
                            )));
                        }
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected global identifier at line: {}",
                            linenum
                        )));
                    }
                }
                bin_vec.push(new_jef("StoreGlobal", &[JEFValue::Int(idx.into())]))
            }
            "pop" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Pop", &[]))
            }
            "box" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("Box", &[]));
            }
            "unbox" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("Unbox", &[]));
            }
            "setbox" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("SetBox", &[]));
            }
            "array" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum)?;
                match arg {
                    Value::Int(size) => {
                        if size > 256 {
                            return Err(AssemblerError::InvalidArgument(format!(
                                "Expected array size <= 256 at line: {}",
                                linenum
                            )));
                        }
                        bin_vec.push(new_jef("Array", &[JEFValue::Int(size)]));
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected numeric array size at line: {}",
                            linenum
                        )));
                    }
                }
            }
            "arrayset" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("ArraySet", &[]));
            }
            "arrayget" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("ArrayGet", &[]));
            }
            "arraypush" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("ArrayPush", &[]));
            }
            "arraypop" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("ArrayPop", &[]));
            }
            "arraylen" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected zero arguments at line: {}",
                        linenum
                    )));
                }
                bin_vec.push(new_jef("ArrayLen", &[]));
            }
            // Control Flow
            "label" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum)?;
                match arg {
                    Value::Ident(name) => {
                        // labels.insert(name, (bin_vec.len() - 1) as u32);
                        bin_vec.push(new_jef("Label", &[JEFValue::String(name)]));
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected label identifier at line: {}",
                            linenum
                        )));
                    }
                }
            }
            "jump" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum)?;
                match arg {
                    Value::Ident(name) => {
                        bin_vec.push(new_jef("Jump", &[JEFValue::String(name)]));
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected label identifier at line: {}",
                            linenum
                        )));
                    }
                }
            }
            "jmpf" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum)?;
                match arg {
                    Value::Ident(name) => {
                        bin_vec.push(new_jef("JumpIfFalse", &[JEFValue::String(name)]));
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected label identifier at line: {}",
                            linenum
                        )));
                    }
                }
            }
            "jmpt" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum)?;
                match arg {
                    Value::Ident(name) => {
                        bin_vec.push(new_jef("JumpIfTrue", &[JEFValue::String(name)]));
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected label identifier at line: {}",
                            linenum
                        )));
                    }
                }
            }

            // Comparisons and operators
            "equl" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Equal", &[]));
            }
            "nteq" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("NotEqual", &[]));
            }
            "lsth" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("LessThan", &[]));
            }
            "grth" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("GreaterThan", &[]));
            }
            "gteq" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("GreaterEqual", &[]));
            }
            "lteq" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("LessEqual", &[]));
            }
            "not" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Not", &[]));
            }
            "and" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("LogicalAnd", &[]));
            }
            "or" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("LogicalOr", &[]));
            }

            // Functions
            "func" => {
                if data.len() != 3 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected two argument".to_string(),
                    ));
                }
                let ident = parse_literal(data[1], linenum)?;
                let arity = parse_literal(data[2], linenum)?;
                if !current_function.done {
                    return Err(AssemblerError::InvalidFunctionLocation(format!(
                        "Cannot create function inside function at line: {}",
                        linenum
                    )));
                }
                match (ident, arity) {
                    (Value::Ident(id), Value::Int(ar)) => {
                        if let Ok(num) = u8::try_from(ar) {
                            func_names.insert(id.clone(), functions.len());

                            functions.push(Function {
                                address: bin_vec.len(),
                                arity: num,
                                locals: 0,
                            });
                            bin_vec.push(new_jef("NoOp", &[]));
                            current_function.done = false;
                            current_function.locals = HashMap::new();
                            current_function.name = id;
                            for n in 0..num {
                                current_function.locals.insert(
                                    format!("arg{}", n.to_string()),
                                    current_function.locals.len() as u8,
                                );
                            }
                        } else {
                            return Err(AssemblerError::InvalidArgument(format!(
                                "Expected arity < 256 at line: {}",
                                linenum
                            )));
                        }
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected arity < 256 and Identifier at line: {}",
                            linenum
                        )));
                    }
                }
            }
            "endf" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                if current_function.done {
                    return Err(AssemblerError::InvalidFunctionEnd(format!(
                        "Tried to end function while not in function at line: {}",
                        linenum
                    )));
                }
                current_function.done = true;
                if let Some(idx) = func_names.get(&current_function.name) {
                    // println!("endf locals: {:?}", functions[*idx].locals);
                    functions[*idx].locals = current_function.locals.len() as u8;
                } else {
                    println!("funcnames: {:?}, {:?}", func_names, current_function.name);
                }
                bin_vec.push(new_jef("Return", &[]));
            }
            "callf" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(format!(
                        "Expected one argument at line: {}",
                        linenum
                    )));
                }
                let val = parse_literal(data[1], linenum)?;
                match val {
                    Value::Ident(ident) => {
                        if let Some(idx) = func_names.get(&ident) {
                            // bin_vec.push(OpCode::CallFunction as u8);
                            // let arg = u16::to_le_bytes(*idx as u16);
                            // bin_vec.push(arg[0]);
                            // bin_vec.push(arg[1]);
                            bin_vec.push(new_jef("CallFunction", &[JEFValue::Int(*idx as i64)]))
                        } else {
                            return Err(AssemblerError::InvalidFunctionCall(format!(
                                "Function doesn't exist at line: {}",
                                linenum
                            )));
                        }
                    }
                    _ => {
                        return Err(AssemblerError::InvalidArgument(format!(
                            "Expected identifier at line: {}",
                            linenum
                        )));
                    }
                }
            }

            // Other
            "prnt" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(new_jef("Print", &[]));
            }
            _ => {
                return Err(AssemblerError::InvalidOpcode(format!(
                    "Invalid OpCode: {}, at line: {}",
                    op, linenum
                )));
            }
        }
    }

    // for label in fix_labels {
    //     if let Some(loc) = labels.get(&label.label) {
    //         let bytes = u32::to_le_bytes(*loc);
    //         bin_vec[label.offset] = bytes[0];
    //         bin_vec[label.offset + 1] = bytes[1];
    //         bin_vec[label.offset + 2] = bytes[2];
    //         bin_vec[label.offset + 3] = bytes[3];
    //     } else {
    //         return Err(AssemblerError::InvalidJumpTarget(format!(
    //             "Invalid jump target: {}",
    //             label.label
    //         )));
    //     }
    // }

    Ok(JEF {
        consts,
        functions,
        code: bin_vec,
    })
}

fn parse_literal(s: &str, line: i32) -> Result<Value, AssemblerError> {
    let arg = s.trim();
    if arg.starts_with('"') && arg.ends_with('"') || arg.starts_with('\'') && arg.ends_with('\'') {
        let content = &arg[1..arg.len() - 1];
        return Ok(Value::String(HeapString::new(content.to_string())));
    }
    if arg == "true" || arg == "false" {
        return Ok(Value::Bool(arg.parse().unwrap()));
    }
    if arg.contains('.') {
        return arg.parse::<f64>().map(|x| Value::Float(x)).map_err(|_| {
            AssemblerError::InvalidLiteral(format!("Invalid Float at line: {}", line))
        });
    }
    // return arg
    //     .parse::<i64>()
    //     .map(|x| Value::Int(x))
    //     .map_err(|_| AssemblerError::InvalidLiteral(format!("Invalid Literal at line: {}", line)));
    let iresult = arg.parse::<i64>();
    match iresult {
        Ok(val) => {
            return Ok(Value::Int(val));
        }
        Err(_) => {
            if let Some(ch) = arg.chars().next() {
                if ch.is_alphabetic() {
                    return Ok(Value::Ident(arg.to_string()));
                } else {
                    return Err(AssemblerError::InvalidLiteral(format!(
                        "Expected Identifier at line: {}",
                        line
                    )));
                }
            } else {
                return Err(AssemblerError::InvalidLiteral(format!(
                    "Invalid literal at line: {}",
                    line
                )));
            };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_string() {
        let result = parse_literal("\"this is a test\"", 0);
        match result {
            Ok(msg) => {
                assert_eq!(
                    msg,
                    Value::String(HeapString::new("this is a test".to_string()))
                );
            }
            _ => {
                panic!("Invalid string parse!");
            }
        }
    }

    #[test]
    fn parse_bool() {
        let result = parse_literal("true", 0);
        assert_eq!(result.unwrap(), Value::Bool(true));
    }

    #[test]
    fn parse_float() {
        let result = parse_literal("456.78", 0);
        assert_eq!(result.unwrap(), Value::Float(456.78))
    }

    #[test]
    fn parse_int() {
        let result = parse_literal("123", 0);
        assert_eq!(result.unwrap(), Value::Int(123));
    }

    #[test]
    fn parse_ident() {
        let result = parse_literal("testIdent", 0);
        assert_eq!(result.unwrap(), Value::Ident("testIdent".to_string()));
    }

    #[test]
    #[should_panic]
    fn parse_fail() {
        let _ = parse_literal("'test", 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_bad_ident() {
        let _ = parse_literal("1test", 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_empty_arg() {
        let _ = parse_literal("", 0).unwrap();
    }
}
