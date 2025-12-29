use crate::error::AssemblerError;
use crate::opcode::OpCode;
use crate::value::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn assemble() -> Result<Vec<u8>, AssemblerError> {
    let file = File::open("program.fasm")?;
    let reader = BufReader::new(file);
    let mut linenum = 0;
    let mut err = false;
    let mut bin_vec: Vec<u8> = Vec::new();
    let mut consts: Vec<Value> = Vec::new();

    for line in reader.lines() {
        linenum += 1;
        let line = line?;
        let data: Vec<&str> = line.split(" ").collect();
        let op = data[0];
        match op {
            "addi" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::AddInt as u8);
            }
            "addf" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::AddFloat as u8);
            }
            "subi" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::SubInt as u8);
            }
            "subf" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::SubFloat as u8);
            }
            "muli" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::MulInt as u8);
            }
            "mulf" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::MulFloat as u8);
            }
            "divi" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::DivInt as u8);
            }
            "divf" => {
                if data.len() > 1 {
                    return Err(AssemblerError::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                bin_vec.push(OpCode::DivFloat as u8);
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

            _ => {
                return Err(AssemblerError::InvalidOpcode(format!(
                    "Invalid OpCode: {}, at line: {}",
                    op, linenum
                )));
            }
        }
    }

    Ok(bin_vec)
}

fn parse_literal(s: &str, line: i32) -> Result<Value, AssemblerError> {
    let arg = s.trim();
    if (arg.starts_with('"') && arg.ends_with('"') || arg.starts_with('\'') && arg.ends_with('\''))
    {
        let content = &arg[1..arg.len() - 1];
        return Ok(Value::String(content.to_string()));
    }
    if arg == "true" || arg == "false" {
        return Ok(Value::Bool(arg.parse().unwrap()));
    }
    if arg.contains('.') {
        return arg.parse::<f64>().map(|x| Value::Float(x)).map_err(|e| {
            AssemblerError::InvalidLiteral(format!("Invalid Float at line: {}", line))
        });
    }
    return arg
        .parse::<i64>()
        .map(|x| Value::Int(x))
        .map_err(|_| AssemblerError::InvalidLiteral(format!("Invalid Literal at line: {}", line)));
}

#[cfg(test)]
mod tests {
    use std::char::ParseCharError;

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
    #[should_panic]
    fn parse_fail() {
        let result = parse_literal("'test", 0).unwrap();
    }
}
