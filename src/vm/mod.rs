mod error;
pub mod opcode;
mod run;
mod vm;

use crate::compiler::compile;
use crate::vm::vm::VM;

pub fn interpret(source: &str) {
    let chunk = compile(source).unwrap();

    let mut vm = VM::new(chunk);
    vm.interpret().unwrap();
}
