use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;
use crate::vm::opcode::Opcode;

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
        }
    }

    pub fn end_compiler(mut self) -> Chunk {
        // TODO: Clones???
        // self.emit_return();

        println!("{}", self.current_chunk());

        self.chunk
    }

    pub fn emit_return(&mut self) {
        // self.emit(Opcode::Nil); // TODO: Return Nil???
        self.emit(Opcode::Return);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.current_chunk().add_constant(value);
        self.emit(Opcode::Constant);
        self.emit_byte(constant);
    }

    pub fn emit(&mut self, opcode: Opcode) {
        self.current_chunk().write(opcode);
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.current_chunk().write_byte(byte);
    }

    pub fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
}
