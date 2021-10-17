pub type LexResult<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedEOF,
    UnexpectedChar,
}
