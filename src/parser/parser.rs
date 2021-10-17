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
            TokenType::Puts => self.parse_puts(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_puts(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Puts)?;
        let expr = self.parse_expr_statement()?;
        Ok(Expr::puts(expr))
    }

    pub fn parse_expr_statement(&mut self) -> ParseResult<Expr> {
        let expr = self.expression()?;
        self.match_(TokenType::Line)?;
        Ok(expr)
    }

    pub fn expression(&mut self) -> ParseResult<Expr> {
        expr_parser::parse(self)
    }

    pub fn expect(&mut self, expect: TokenType) -> ParseResult<Token<'a>> {
        if self.check(expect.clone())? {
            // TODO: Clone
            return Ok(self.consume()?);
        }

        Err(ParserError::Expected(
            expect.clone(),                         // TODO: Clone
            self.peek_type()?.clone(),              // TODO: Clone
            self.peek()?.position().line().clone(), // TODO: Clone
        ))
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
