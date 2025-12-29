use fvm::assembler::assemble;
use fvm::bytecode::Bytecode;
use fvm::vm::VM;
use std::time::{Duration, Instant};

fn main() {
    let start: Instant;
    let end: Duration;
    let mut vm = VM::new(256);
    let assembled = assemble();
    match assembled {
        Ok(vec) => {
            println!("{:?}", vec);
            let consts = vec.0;
            let code = vec.1;
            let bytecode = Bytecode { consts, code };
            vm.load_code(bytecode);
            start = Instant::now();
            let result = vm.execute();
            end = start.elapsed();

            match result {
                Ok(_) => {
                    println!("VM Returned OK.");
                    println!("Runtime: {:.8?}", end);
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
