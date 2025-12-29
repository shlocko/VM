use fvm::assembler::assemble;
use fvm::bytecode::Bytecode;
use fvm::value::Value;
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
            _ = vm.execute();
        }
        Err(er) => {
            println!("Error");
        }
    }
}
