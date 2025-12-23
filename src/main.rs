mod bytecode;
mod memory;
mod opcode;
mod value;
mod vm;

use bytecode::Bytecode;
use vm::VM;

fn main() {
    let bytecode = Bytecode {
        consts: Vec::new(),
        code: vec![0x10, 3, 0x10, 4, 0x00],
    };

    let mut vm = VM::new(256);
    vm.load_code(bytecode);
    let _ = vm.execute();
}
