#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    token_type: TokenType,
    source: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, source: &'a str, position: Position) -> Self {
        Token {
            token_type,
            source,
            position,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,
    Slash,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    True,
    False,
    Puts,
    Identifier,

    Number,

    Line,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Position {
    start: usize,
    end: usize,
    line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Position { start, end, line }
    }

    pub fn start(&self) -> &usize {
        &self.start
    }

    pub fn end(&self) -> &usize {
        &self.end
    }

    pub fn line(&self) -> &usize {
        &self.line
    }
}

pub trait ToKeyword {
    fn to_keyword(self) -> TokenType;
}

impl ToKeyword for &str {
    fn to_keyword(self) -> TokenType {
        match self {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "puts" => TokenType::Puts,
            _ => return TokenType::Identifier,
        }
    }
}
