use crate::lexer::error::{LexResult, SyntaxError};
use crate::lexer::token::{Position, ToKeyword, Token, TokenType};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
        }
    }

    pub fn read_token(&mut self) -> LexResult<Option<Token<'a>>> {
        self.skip_whitespace()?;
        if self.is_at_end() {
            return self.eof();
        }

        let (start, c) = self.advance()?;

        if c.is_alphabetic() {
            return self.identifier(start);
        }
        if c.is_digit(10) {
            return self.number(start);
        }

        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => {
                // Ignore comments.
                if self.check('/')? {
                    self.advance_while(|&ch| ch != '\n')?;
                    return Ok(None);
                } else {
                    TokenType::Slash
                }
            }
            '\n' | '\r' => TokenType::Line,
            '=' => TokenType::Equal,
            _ => {
                return Err(SyntaxError::UnexpectedChar);
            }
        };

        self.make_token(token_type, start)
    }

    fn identifier(&mut self, start: usize) -> LexResult<Option<Token<'a>>> {
        self.advance_while(|&c| c.is_alphanumeric())?;
        let keyword = self.token_contents(start).to_keyword();
        self.make_token(keyword, start)
    }

    fn number(&mut self, start: usize) -> LexResult<Option<Token<'a>>> {
        self.advance_while(|c| c.is_digit(10))?;

        // Look for a fractional part
        if let Some(peek) = self.peek() {
            if peek == '.' {
                if let Some(next) = self.peek_next() {
                    if next.is_digit(10) {
                        // Consume the '.'.
                        self.advance()?;

                        self.advance_while(|c| c.is_digit(10))?;
                    }
                }
            }
        }

        self.make_token(TokenType::Number, start)
    }

    fn eof(&mut self) -> LexResult<Option<Token<'a>>> {
        self.make_token(TokenType::EOF, self.source.len())
    }

    fn make_token(&mut self, token_type: TokenType, start: usize) -> LexResult<Option<Token<'a>>> {
        let source = self.token_contents(start);
        let pos = Position::new(start, start + source.len(), self.line);
        Ok(Some(Token::new(token_type, source, pos)))
    }

    fn token_contents(&mut self, start: usize) -> &'a str {
        let end = self
            .chars
            .peek()
            .map(|&(i, _)| i)
            .unwrap_or(self.source.len());
        &self.source[start..end].trim_end()
    }

    fn skip_whitespace(&mut self) -> LexResult<()> {
        self.advance_while(|&c| c == ' ' || c == '\t' || c == '\n' || c == '\r')?;
        Ok(())
    }

    fn advance_while<F>(&mut self, f: F) -> LexResult<usize>
    where
        for<'r> F: Fn(&'r char) -> bool,
    {
        let mut count = 0;
        while let Some(char) = self.peek() {
            if f(&char) {
                self.advance()?;
                count += 1;
            } else {
                break;
            }
        }
        Ok(count)
    }

    fn advance(&mut self) -> LexResult<(usize, char)> {
        self.chars
            .next()
            .map(|(current, c)| {
                if c == '\n' {
                    self.line += 1;
                }
                (current, c)
            })
            .ok_or(SyntaxError::UnexpectedEOF)
    }

    fn check(&mut self, c: char) -> LexResult<bool> {
        self.peek()
            .map(|p| p == c)
            .ok_or(SyntaxError::UnexpectedEOF)
    }

    fn peek_next(&mut self) -> Option<char> {
        self.chars.nth(1).map(|(_, c)| c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}
