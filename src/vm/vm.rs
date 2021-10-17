use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;
use crate::vm::error::{RunResult, RuntimeError};
use std::io::{stdout, Stdout, Write};

pub struct VM<W: Write> {
    stack: Vec<Value>,
    pub ip: usize,
    chunk: Chunk,
    stdout: W,
}

impl VM<Stdout> {
    pub fn new(chunk: Chunk) -> Self {
        VM::with_stdout(stdout(), chunk)
    }
}

impl<W: Write> VM<W> {
    pub fn with_stdout(stdout: W, chunk: Chunk) -> Self {
        VM {
            stack: Vec::with_capacity(u8::MAX as usize),
            ip: 0,
            chunk,
            stdout,
        }
    }

    pub fn interpret(&mut self) -> RunResult<()> {
        self.run()
    }

    pub fn read_constant(&mut self) -> RunResult<&Value> {
        let constant_index = self.read_byte()?;
        Ok(self.current_chunk()?.read_constant(constant_index as usize))
    }

    pub fn read_byte(&mut self) -> RunResult<u8> {
        let byte = self.chunk.code()[self.ip];
        self.ip += 1;
        Ok(byte)
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn peek(&mut self) -> RunResult<&Value> {
        self.stack.last().ok_or(RuntimeError::StackEmpty)
    }

    pub fn pop(&mut self) -> RunResult<Value> {
        self.stack.pop().ok_or(RuntimeError::StackEmpty)
    }

    pub fn is_at_end(&self) -> RunResult<bool> {
        Ok(self.ip >= self.current_chunk()?.code().len())
    }

    fn current_chunk(&self) -> RunResult<&Chunk> {
        Ok(&self.chunk)
    }
}
