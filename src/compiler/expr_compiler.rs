use crate::compiler::compiler::Compiler;
use crate::compiler::instance::CompilerInstance;
use crate::compiler::object::{FunctionType, Gc};
use crate::compiler::value::Value;
use crate::parser::ast::*;
use crate::vm::opcode::Opcode;

pub fn compile_expr(c: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Binary { left, op, right } => compile_binary(c, left, op, right),
        Expr::Fun { name, decl } => compile_fun(c, name, decl),
        Expr::Call { callee, args } => compile_call(c, callee, args),
        Expr::VarSet { name, value } => compile_var_set(c, name, value),
        Expr::VarGet { name } => compile_var_get(c, name),
        Expr::Block { block } => compile_block(c, block),
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

fn compile_fun(compiler: &mut Compiler, name: Identifier, decl: FunDecl) {
    compiler.set_instance(CompilerInstance::new(FunctionType::Function));
    compile_closure(compiler, &name, decl);
    compiler.define_variable(&name);
}

fn compile_closure(compiler: &mut Compiler, name: &Identifier, decl: FunDecl) {
    compiler.begin_scope();

    let arity = decl.args().len();

    // Compile function arguments.
    for arg in decl.args() {
        compiler.declare_variable(arg);
        compiler.define_variable(arg);
    }

    // Compile function body.
    compile_expr(compiler, Expr::block(decl.body()));

    // Create closure object.
    let mut closure = compiler.end_compiler();
    closure.set_name(name.clone());
    closure.set_arity(arity as u8);

    compiler.emit(Opcode::Closure);

    let constant_id = compiler.add_constant(Value::Function(Gc::new(closure)));
    compiler.emit_byte(constant_id);
}

fn compile_call(compiler: &mut Compiler, callee: Box<Expr>, args: Vec<Expr>) {
    let arity = args.len();
    compile_expr(compiler, *callee);
    for arg in args {
        compile_expr(compiler, arg);
    }
    compiler.emit(Opcode::Call);
    compiler.emit_byte(arity as u8);
}

fn compile_var_set(compiler: &mut Compiler, name: Identifier, value: Box<Expr>) {
    compile_expr(compiler, *value);

    compiler.emit(Opcode::SetGlobal);
    let constant_id = compiler
        .current_chunk()
        .add_constant(Value::String(name.to_string()));
    compiler.emit_byte(constant_id);
}

fn compile_var_get(compiler: &mut Compiler, name: Identifier) {
    compiler.emit(Opcode::GetGlobal);
    let constant_id = compiler.add_constant(Value::String(name));
    compiler.emit_byte(constant_id);
}

fn compile_block(compiler: &mut Compiler, block: Box<BlockDecl>) {
    compiler.begin_scope();
    for expr in *block {
        compile_expr(compiler, expr);
    }
    compiler.end_scope();
}

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
