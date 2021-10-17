use crate::vm::error::RunResult;
use crate::vm::opcode::Opcode;
use crate::vm::vm::VM;
use std::io::Write;

impl<W: Write> VM<W> {
    pub fn run(&mut self) -> RunResult<()> {
        while !self.is_at_end()? {
            let instruction = Opcode::from(self.read_byte()?);
            match instruction {
                Opcode::Constant => self.constant()?,
                Opcode::Add => self.add()?,
                Opcode::Subtract => self.subtract()?,
                Opcode::Multiply => self.multiply()?,
                Opcode::Divide => self.divide()?,
                Opcode::Return => self.ret()?,
            }
        }
        Ok(())
    }

    fn constant(&mut self) -> RunResult<()> {
        let constant = self.read_constant()?.clone();
        self.push(constant);
        Ok(())
    }

    fn add(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a + b);
        Ok(())
    }

    fn subtract(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a - b);
        Ok(())
    }

    fn multiply(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a * b);
        Ok(())
    }

    fn divide(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a / b);
        Ok(())
    }

    fn ret(&mut self) -> RunResult<()> {
        println!("{}", self.pop()?);
        Ok(())
    }
}
