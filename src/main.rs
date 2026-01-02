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
        Ok(vec) => {
            println!("{:?}", vec);
            let entry = vec.0;
            let consts = vec.1;
            let functions = vec.2;
            let code = vec.3;
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
