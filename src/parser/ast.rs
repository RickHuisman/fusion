use crate::lexer::token::TokenType;
use crate::parser::error::{ParseResult, ParserError};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    VarSet {
        name: Identifier,
        value: Box<Expr>,
    },
    VarGet {
        name: Identifier,
    },
    Puts {
        value: Box<Expr>,
    },
    Literal(LiteralExpr),
}

impl Expr {
    pub fn binary(left: Expr, op: BinaryOperator, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn var_set(name: Identifier, value: Expr) -> Self {
        Expr::VarSet {
            name,
            value: Box::new(value),
        }
    }

    pub fn var_get(name: Identifier) -> Self {
        Expr::VarGet { name }
    }

    pub fn number(n: f64) -> Expr {
        Expr::Literal(LiteralExpr::Number(n))
    }

    pub fn true_() -> Expr {
        Expr::Literal(LiteralExpr::True)
    }

    pub fn false_() -> Expr {
        Expr::Literal(LiteralExpr::False)
    }

    pub fn puts(value: Expr) -> Self {
        Expr::Puts {
            value: Box::new(value),
        }
    }
}

pub type Identifier = String;

#[derive(PartialEq, Debug)]
pub enum LiteralExpr {
    Number(f64),
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOperator {
    Equal,
    BangEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Subtract,
    Add,
    Divide,
    Multiply,
}

impl BinaryOperator {
    pub fn from_token(token_type: &TokenType) -> ParseResult<BinaryOperator> {
        Ok(match token_type {
            TokenType::Minus => BinaryOperator::Subtract,
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Star => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::Equal,
            TokenType::LessThan => BinaryOperator::LessThan,
            TokenType::LessThanEqual => BinaryOperator::LessThanEqual,
            TokenType::GreaterThan => BinaryOperator::GreaterThan,
            TokenType::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            _ => return Err(ParserError::ExpectedBinaryOperator(token_type.clone())),
        })
    }
}
