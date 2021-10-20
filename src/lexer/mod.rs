use crate::lexer::error::LexResult;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

mod error;
mod lexer;
pub mod morph;
pub mod token;

pub fn lex(source: &str) -> LexResult<Vec<Token>> {
    let mut lexer = Lexer::new(source);

    let mut tokens = vec![];
    while let Some(token) = lexer.read_token()? {
        if let TokenType::EOF = token.token_type() {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    Ok(tokens)
}
