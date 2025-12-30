use crate::error::AssemblerError;
use crate::opcode::OpCode;
use crate::value::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct FixLabel {
    offset: usize,
    label: String,
}

pub fn assemble() -> Result<(Vec<Value>, Vec<u8>), AssemblerError> {
    let file = File::open("program.fasm")?;
    let reader = BufReader::new(file);
    let mut linenum = 0;
    let mut bin_vec: Vec<u8> = Vec::new();
    let mut consts: Vec<Value> = Vec::new();
    let mut globals_names: HashMap<String, u16> = HashMap::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut fix_labels: Vec<FixLabel> = Vec::new();

    for line in reader.lines() {
        linenum += 1;
        let line = line?;
        let data: Vec<&str> = line.split(" ").collect();
        let op = data[0];
        match op {
            ";" => {}
            "" => {}
            "adds" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::Add as u8);
            }
            "subs" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::Sub as u8);
            }
            "muls" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::Mul as u8);
            }
            "divs" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::Div as u8);
            }
            "divi" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::DivInt as u8);
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
                        let index = consts.iter().position(|x| *x == val);
                        match index {
                            Some(val) => {
                                idx = val as u16;
                            }
                            None => {
                                consts.push(val);
                                idx = (consts.len() - 1) as u16;
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
                let final_arg = idx.to_le_bytes();
                bin_vec.push(OpCode::PushConst as u8);
                bin_vec.push(final_arg[0]);
                bin_vec.push(final_arg[1]);
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
                let final_arg = idx.to_le_bytes();
                bin_vec.push(OpCode::StoreGlobal as u8);
                bin_vec.push(final_arg[0]);
                bin_vec.push(final_arg[1]);
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
                let final_arg = u16::to_le_bytes(idx);
                bin_vec.push(OpCode::PushGlobal as u8);
                bin_vec.push(final_arg[0]);
                bin_vec.push(final_arg[1]);
            }

            // Control Flow
            "labl" => {
                if data.len() != 2 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg = parse_literal(data[1], linenum)?;
                bin_vec.push(OpCode::NoOp as u8);
                match arg {
                    Value::Ident(name) => {
                        labels.insert(name, (bin_vec.len() - 1) as u32);
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
                        // labels.insert(name, (bin_vec.len() - 1) as u32);
                        bin_vec.push(OpCode::Jump as u8);
                        if let Some(target) = labels.get(&name) {
                            let location = u32::to_le_bytes(*target);
                            bin_vec.push(location[0]);
                            bin_vec.push(location[1]);
                            bin_vec.push(location[2]);
                            bin_vec.push(location[3]);
                        } else {
                            fix_labels.push(FixLabel {
                                offset: bin_vec.len(),
                                label: name,
                            });
                            let location = u32::to_le_bytes(0);
                            bin_vec.push(location[0]);
                            bin_vec.push(location[1]);
                            bin_vec.push(location[2]);
                            bin_vec.push(location[3]);
                        };
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
                        // labels.insert(name, (bin_vec.len() - 1) as u32);
                        bin_vec.push(OpCode::JumpIfFalse as u8);
                        if let Some(target) = labels.get(&name) {
                            let location = u32::to_le_bytes(*target);
                            bin_vec.push(location[0]);
                            bin_vec.push(location[1]);
                            bin_vec.push(location[2]);
                            bin_vec.push(location[3]);
                        } else {
                            fix_labels.push(FixLabel {
                                offset: bin_vec.len(),
                                label: name,
                            });
                            let location = u32::to_le_bytes(0);
                            bin_vec.push(location[0]);
                            bin_vec.push(location[1]);
                            bin_vec.push(location[2]);
                            bin_vec.push(location[3]);
                        };
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
                        // labels.insert(name, (bin_vec.len() - 1) as u32);
                        bin_vec.push(OpCode::JumpIfTrue as u8);
                        if let Some(target) = labels.get(&name) {
                            let location = u32::to_le_bytes(*target);
                            bin_vec.push(location[0]);
                            bin_vec.push(location[1]);
                            bin_vec.push(location[2]);
                            bin_vec.push(location[3]);
                        } else {
                            fix_labels.push(FixLabel {
                                offset: bin_vec.len(),
                                label: name,
                            });
                            let location = u32::to_le_bytes(0);
                            bin_vec.push(location[0]);
                            bin_vec.push(location[1]);
                            bin_vec.push(location[2]);
                            bin_vec.push(location[3]);
                        };
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
                bin_vec.push(OpCode::Equal as u8);
            }
            "nteq" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::NotEqual as u8);
            }
            "lsth" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::LessThan as u8);
            }
            "grth" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::GreaterThan as u8);
            }
            "gteq" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::GreaterEqual as u8);
            }
            "lteq" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::LessEqual as u8);
            }
            "bnot" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::Not as u8);
            }
            "land" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::LogicalAnd as u8);
            }
            "lgor" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::LogicalOr as u8);
            }

            "prnt" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::Print as u8);
            }
            _ => {
                return Err(AssemblerError::InvalidOpcode(format!(
                    "Invalid OpCode: {}, at line: {}",
                    op, linenum
                )));
            }
        }
    }

    for label in fix_labels {
        if let Some(loc) = labels.get(&label.label) {
            let bytes = u32::to_le_bytes(*loc);
            bin_vec[label.offset] = bytes[0];
            bin_vec[label.offset + 1] = bytes[1];
            bin_vec[label.offset + 2] = bytes[2];
            bin_vec[label.offset + 3] = bytes[3];
        } else {
            return Err(AssemblerError::InvalidJumpTarget(format!(
                "Invalid jump target: {}",
                label.label
            )));
        }
    }

    Ok((consts, bin_vec))
}

fn parse_literal(s: &str, line: i32) -> Result<Value, AssemblerError> {
    let arg = s.trim();
    if arg.starts_with('"') && arg.ends_with('"') || arg.starts_with('\'') && arg.ends_with('\'') {
        let content = &arg[1..arg.len() - 1];
        return Ok(Value::String(content.to_string()));
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
                assert_eq!(msg, Value::String("this is a test".to_string()));
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
