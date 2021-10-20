mod error;
mod frame;
mod gc;
pub mod opcode;
mod run;
mod vm;

use crate::compiler::compile;
use crate::vm::vm::VM;

pub fn interpret(source: &str) {
    let fun = compile(source).unwrap();

    let mut vm = VM::new();
    vm.interpret(fun).unwrap();
}
