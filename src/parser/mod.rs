use crate::lexer::token::Token;
use crate::parser::ast::Expr;
use crate::parser::error::ParseResult;
use crate::parser::parser::Parser;

pub(crate) mod ast;
mod error;
mod expr_parser;
mod parser;

pub fn parse<'a>(tokens: &'a mut Vec<Token<'a>>) -> ParseResult<Vec<Expr>> {
    let mut parser = Parser::new(tokens);

    let mut ast = vec![];
    while !parser.is_eof()? {
        ast.push(parser.parse_top_level_expr()?);
    }

    Ok(ast)
}
