#[repr(u8)]
pub enum Opcode {
    Return,
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    SetGlobal,
    GetGlobal,
    GetLocal,
    SetLocal,
    Closure,
    Call,
    Puts,
    Pop,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Return,
            0x01 => Opcode::Constant,
            0x02 => Opcode::Add,
            0x03 => Opcode::Subtract,
            0x04 => Opcode::Multiply,
            0x05 => Opcode::Divide,
            0x06 => Opcode::SetGlobal,
            0x07 => Opcode::GetGlobal,
            0x08 => Opcode::SetLocal,
            0x09 => Opcode::GetLocal,
            0x0a => Opcode::Closure,
            0x0b => Opcode::Call,
            0x0c => Opcode::Puts,
            0x0d => Opcode::Pop,
            _ => panic!("No opcode for byte: {}", byte),
        }
    }
}
