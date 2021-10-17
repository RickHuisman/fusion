use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::Expr;
use crate::parser::error::{ParseResult, ParserError};
use crate::parser::expr_parser;

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token<'a>>) -> Self {
        tokens.reverse();
        Parser { tokens }
    }

    pub fn parse_top_level_expr(&mut self) -> ParseResult<Expr> {
        match self.peek_type()? {
            _ => self.parse_expr_statement(),
        }
    }

    pub fn parse_expr_statement(&mut self) -> ParseResult<Expr> {
        let expr = self.expression()?;
        // self.match_(TokenType::Semicolon)?; TODO
        Ok(expr)
    }

    pub fn expression(&mut self) -> ParseResult<Expr> {
        expr_parser::parse(self)
    }

    pub fn consume(&mut self) -> ParseResult<Token<'a>> {
        self.tokens.pop().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek(&self) -> ParseResult<&Token<'a>> {
        self.tokens.last().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek_type(&self) -> ParseResult<&TokenType> {
        Ok(self.peek()?.token_type())
    }

    pub fn match_(&mut self, token_type: TokenType) -> ParseResult<bool> {
        if !self.check(token_type)? {
            return Ok(false);
        }

        self.consume()?;
        Ok(true)
    }

    pub fn check(&self, token_type: TokenType) -> ParseResult<bool> {
        Ok(self.peek_type()? == &token_type)
    }

    pub fn is_eof(&self) -> ParseResult<bool> {
        Ok(self.check(TokenType::EOF)?)
    }
}
