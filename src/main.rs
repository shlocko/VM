use fvm::assembler::assemble;
use fvm::bytecode::Bytecode;
use fvm::value::Value;
use fvm::vm::VM;

fn main() {
    let arg1 = u16::to_le_bytes(5);
    let arg2 = u16::to_le_bytes(6);
    let bytecode = Bytecode {
        consts: vec![Value::Int(3), Value::Int(2)],
        code: vec![0x16, arg1[0], arg1[1], 0x10, arg2[0], arg2[1], 0x00],
        // code: vec![0x10, 0x00, 0x00, 0x10, 0x01, 0x00, 0x00], // handcoded arguments in LE
    };

    let mut vm = VM::new(256);
    vm.load_code(bytecode);
    // let _ = vm.execute();
    let assembled = assemble();
    match assembled {
        Ok(vec) => {
            println!("{:?}", vec);
        }
        Err(er) => {
            println!("Error");
        }
    }
}
