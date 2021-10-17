use crate::compiler::compiler::Compiler;
use crate::compiler::value::Value;
use crate::parser::ast::*;
use crate::vm::opcode::Opcode;

pub fn compile_expr(c: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Binary { left, op, right } => compile_binary(c, left, op, right),
        Expr::Number(n) => compile_number(c, n),
    }
}

fn compile_binary(compiler: &mut Compiler, left: Box<Expr>, op: BinaryOperator, right: Box<Expr>) {
    compile_expr(compiler, *left);
    compile_expr(compiler, *right);

    match op {
        BinaryOperator::Add => compiler.emit(Opcode::Add),
        BinaryOperator::Subtract => compiler.emit(Opcode::Subtract),
        BinaryOperator::Multiply => compiler.emit(Opcode::Multiply),
        BinaryOperator::Divide => compiler.emit(Opcode::Divide),
        _ => todo!(),
    }
}

fn compile_number(compiler: &mut Compiler, number: f64) {
    compiler.emit_constant(Value::Number(number));
}
