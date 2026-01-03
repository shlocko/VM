use fvm::bytecode::Bytecode;
use fvm::vm::VM;
use fvm::{assembler::assemble, value::Value};
use std::time::{Duration, Instant};

fn main() {
    let start: Instant;
    let end: Duration;
    let mut vm = VM::new(256);
    let assembled = assemble();
    match assembled {
        Ok(data) => {
            println!("{:?}", data);
            let entry = data.0;
            let consts = data.1;
            let functions = data.2;
            let code = data.3;
            let bytecode = Bytecode {
                entry,
                consts,
                functions,
                code,
            };
            vm.load_code(bytecode);
            start = Instant::now();
            let result = vm.execute();
            end = start.elapsed();

            match result {
                Ok(_) => {
                    println!("VM Returned OK.");
                    println!("Runtime: {:.8?}", end);
                    println!("enum size: {}bytes", size_of::<Value>())
                }
                Err(e) => {
                    println!("VM Returned Error: {:?}", e);
                }
            }
        }
        Err(er) => {
            println!("Error {:?}", er);
        }
    }
}
