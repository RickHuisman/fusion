use crate::compiler::value::Value;
use crate::vm::opcode::Opcode;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write(&mut self, opcode: Opcode) {
        self.write_byte(opcode as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        // TODO: u8 or usize?
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn code_mut(&mut self) -> &mut Vec<u8> {
        &mut self.code
    }

    pub fn read_constant(&self, index: usize) -> &Value {
        &self.constants[index]
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut offset = 0;
        while offset < self.code.len() {
            offset = disassemble_instruction(f, self, &mut offset);
        }

        write!(f, "")
    }
}

fn disassemble_instruction(f: &mut Formatter<'_>, chunk: &Chunk, offset: &mut usize) -> usize {
    write!(f, "{:04X}", offset);

    write!(f, "   | ");

    let instruction = Opcode::from(chunk.code[*offset]);
    match instruction {
        Opcode::Return => simple_instruction(f, "RETURN", offset),
        Opcode::Constant => constant_instruction(chunk, f, "CONSTANT", offset),
        Opcode::Add => simple_instruction(f, "ADD", offset),
        Opcode::Subtract => simple_instruction(f, "SUBTRACT", offset),
        Opcode::Multiply => simple_instruction(f, "MULTIPLY", offset),
        Opcode::Divide => simple_instruction(f, "DIVIDE", offset),
        Opcode::SetGlobal => constant_instruction(chunk, f, "SET_GLOBAL", offset),
        Opcode::GetGlobal => constant_instruction(chunk, f, "GET_GLOBAL", offset),
        Opcode::GetLocal => byte_instruction(chunk, f, "GET_LOCAL", offset),
        Opcode::SetLocal => byte_instruction(chunk, f, "SET_LOCAL", offset),
        Opcode::Puts => simple_instruction(f, "PUTS", offset),
        Opcode::Closure => {
            // TODO: Clean up.
            *offset += 2;

            let constant = chunk.code[*offset - 1];
            write!(f, "{:-16} {:4} ", "CLOSURE", constant);
            writeln!(f, "'{:?}'", chunk.constants()[constant as usize]);

            *offset
        }
        Opcode::Call => byte_instruction(chunk, f, "CALL", offset),
        Opcode::Pop => simple_instruction(f, "POP", offset),
    }
}

fn simple_instruction(f: &mut Formatter<'_>, name: &str, offset: &mut usize) -> usize {
    writeln!(f, "{}", name);
    *offset + 1
}

fn constant_instruction(
    chunk: &Chunk,
    f: &mut Formatter<'_>,
    name: &str,
    offset: &mut usize,
) -> usize {
    let constant = chunk.code()[*offset + 1];
    write!(f, "{:-16} {:4} ", name, constant);
    writeln!(f, "'{}'", chunk.constants()[constant as usize]);
    *offset + 2
}

fn byte_instruction(chunk: &Chunk, f: &mut Formatter<'_>, name: &str, offset: &mut usize) -> usize {
    let slot = chunk.code[*offset + 1];
    writeln!(f, "{:-16} {:4X}", name, slot);
    *offset + 2
}
