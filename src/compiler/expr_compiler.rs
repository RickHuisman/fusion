use crate::compiler::compiler::Compiler;
use crate::compiler::value::Value;
use crate::parser::ast::*;
use crate::vm::opcode::Opcode;

pub fn compile_expr(c: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Binary { left, op, right } => compile_binary(c, left, op, right),
        Expr::VarSet { name, value } => compile_var_set(c, name, value),
        Expr::VarGet { name } => compile_var_get(c, name),
        Expr::Literal(l) => compile_literal(c, l),
        Expr::Puts { value } => compile_puts(c, value),
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

fn compile_var_set(compiler: &mut Compiler, name: Identifier, value: Box<Expr>) {
    compile_expr(compiler, *value);

    compiler.emit(Opcode::SetGlobal);
    let constant_id = compiler
        .current_chunk()
        .add_constant(Value::String(name.to_string()));
    compiler.emit_byte(constant_id);
}

fn compile_var_get(compiler: &mut Compiler, name: Identifier) {}

fn compile_literal(compiler: &mut Compiler, literal: LiteralExpr) {
    match literal {
        LiteralExpr::Number(n) => compiler.emit_constant(Value::Number(n)),
        LiteralExpr::True => compiler.emit_constant(Value::Bool(true)),
        LiteralExpr::False => compiler.emit_constant(Value::Bool(false)),
    }
}

fn compile_puts(compiler: &mut Compiler, value: Box<Expr>) {
    compile_expr(compiler, *value);
    compiler.emit(Opcode::Puts);
}
