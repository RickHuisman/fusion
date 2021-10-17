pub type RunResult<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    StackEmpty,
    ArgumentTypes,
    BadStackIndex(usize, usize),
}
