use crate::error::AssemblerErrorKind;
use crate::opcode::OpCode;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn assemble() -> Result<Vec<u8>, AssemblerErrorKind> {
    let file = File::open("program.fasm")?;
    let reader = BufReader::new(file);
    let mut linenum = 0;
    let mut err = false;
    let mut binVec: Vec<u8> = Vec::new();

    for line in reader.lines() {
        linenum += 1;
        let line = line?;
        let data: Vec<&str> = line.split(" ").collect();
        let op = data[0];
        match op {
            "addi" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::AddInt as u8);
            }
            "addf" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::AddFloat as u8);
            }
            "subi" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::SubInt as u8);
            }
            "subf" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::SubFloat as u8);
            }
            "muli" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::MulInt as u8);
            }
            "mulf" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::MulFloat as u8);
            }
            "divi" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::DivInt as u8);
            }
            "divf" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::DivFloat as u8);
            }
            "pshc" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::PushConst as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "pshl" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::PushLocal as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "crtl" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::CreateLocal as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "strl" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::StoreLocal as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "pshg" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::PushGlobal as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "crtg" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::CreateGlobal as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "strg" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<u16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::StoreGlobal as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }
            "pops" => {
                if data.len() > 1 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected zero arguments".to_string(),
                    ));
                }
                binVec.push(OpCode::Pop as u8);
            }
            "pshm" => {
                if data.len() != 2 {
                    return Err(AssemblerErrorKind::InvalidArgument(
                        "Expected one argument".to_string(),
                    ));
                }
                let arg_bytes: [u8; 2];
                if let Ok(arg) = data[1].parse::<i16>() {
                    arg_bytes = arg.to_le_bytes();
                } else {
                    return Err(AssemblerErrorKind::InvalidArgument(format!(
                        "Expected u16 argument, got {}.",
                        data[1]
                    )));
                }
                binVec.push(OpCode::PushImmediate as u8);
                binVec.push(arg_bytes[0]);
                binVec.push(arg_bytes[1]);
            }

            _ => {
                return Err(AssemblerErrorKind::InvalidOpcode(format!(
                    "Invalid OpCode: {}, at line: {}",
                    op, linenum
                )));
            }
        }
    }

    Ok(binVec)
}
