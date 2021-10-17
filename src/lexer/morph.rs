use crate::lexer::token::{Token, TokenType};

pub fn morph(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut morphed = vec![];

    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token.token_type() {
            TokenType::Line => {
                if morphed.is_empty() {
                    morphed.push(token);
                } else {
                    let last_token_type = morphed.last().unwrap().token_type();
                    if last_token_type != &TokenType::Line {
                        morphed.push(token);
                    }
                }
            }
            _ => morphed.push(token),
        }
    }

    morphed.reverse();

    morphed
}
