use crate::compiler::chunk::Chunk;
use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompileResult;
use crate::compiler::expr_compiler::compile_expr;
use crate::lexer::lex;
use crate::parser::parse;
use crate::lexer::morph::morph;

pub mod chunk;
mod compiler;
mod error;
mod expr_compiler;
pub mod value;

pub fn compile(source: &str) -> CompileResult<Chunk> {
    // TODO: Report errors.
    let mut tokens = lex(source).unwrap();
    tokens = morph(tokens);
    let ast = parse(&mut tokens).unwrap();

    let mut compiler = Compiler::new();

    for expr in ast {
        compile_expr(&mut compiler, expr);
    }

    Ok(compiler.end_compiler())
}
