use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

// TODO: Return errors, not panic.
impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b + a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b - a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b * a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b / a),
            _ => panic!("Operand must be a number."),
        }
    }
}