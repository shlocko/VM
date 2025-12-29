use fvm::assembler::assemble;
use fvm::bytecode::Bytecode;
use fvm::vm::VM;

fn main() {
    let mut vm = VM::new(256);
    let assembled = assemble();
    match assembled {
        Ok(vec) => {
            println!("{:?}", vec);
            let consts = vec.0;
            let code = vec.1;
            let bytecode = Bytecode { consts, code };
            vm.load_code(bytecode);
            let result = vm.execute();
            match result {
                Ok(_) => {
                    println!("VM Returned OK.");
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
