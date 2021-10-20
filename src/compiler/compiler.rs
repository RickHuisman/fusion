use crate::compiler::chunk::Chunk;
use crate::compiler::error::CompilerError;
use crate::compiler::instance::CompilerInstance;
use crate::compiler::object::{Function, FunctionType};
use crate::compiler::value::Value;
use crate::parser::ast::Identifier;
use crate::vm::opcode::Opcode;

pub struct Compiler {
    current: CompilerInstance,
    errors: Vec<CompilerError>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            current: CompilerInstance::new(FunctionType::Script),
            errors: vec![],
        }
    }

    pub fn declare_variable(&mut self, ident: &Identifier) {
        if self.is_scoped() {
            if self.contains_local_in_current_scope(&ident) {
                self.add_error(CompilerError::LocalAlreadyDefined);
            }

            self.add_local(&ident);
        }
    }

    pub fn define_variable(&mut self, name: &Identifier) {
        if self.is_scoped() {
            self.mark_local_initialized();
            return;
        }

        self.emit(Opcode::SetGlobal);
        let constant_id = self
            .current_chunk()
            .add_constant(Value::String(name.to_string()));
        self.emit_byte(constant_id);
    }

    pub fn begin_scope(&mut self) {
        self.current.locals_mut().begin_scope();
    }

    pub fn end_scope(&mut self) {
        for _ in self.current.locals_mut().end_scope().iter().rev() {
            self.emit(Opcode::Pop);
        }
    }

    pub fn is_scoped(&self) -> bool {
        self.current.locals().scope_depth() > 0
    }

    pub fn end_compiler(&mut self) -> Function {
        // TODO: Clones???
        self.emit_return();
        let fun_copy = self.current.function().clone();

        println!("{}", self.current_chunk());

        if let Some(enclosing) = *self.current.enclosing().clone() {
            self.current = enclosing;
        }
        fun_copy
    }

    pub fn add_local(&mut self, ident: &Identifier) {
        self.current.locals_mut().insert(ident);
    }

    // TODO: Rename.
    pub fn contains_local_in_current_scope(&self, name: &str) -> bool {
        self.current.locals().get_at_current_depth(name).is_some()
    }

    pub fn mark_local_initialized(&mut self) {
        if !self.is_scoped() {
            return;
        }

        self.current.locals_mut().mark_initialized();
    }

    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.current_chunk().add_constant(value)
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

    pub fn set_instance(&mut self, instance: CompilerInstance) {
        let current_copy = self.current.clone();
        self.current = instance;
        *self.current.enclosing_mut() = Box::new(Some(current_copy));
    }

    pub fn current_chunk(&mut self) -> &mut Chunk {
        self.current.function_mut().chunk_mut()
    }
}
