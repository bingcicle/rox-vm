#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum OpCode {
    OP_RETURN = 0,
    OP_CONSTANT = 1,
    OP_ADD = 2,
    OP_SUBTRACT = 3,
    OP_MULTIPLY = 4,
    OP_DIVIDE = 5,
    OP_NEGATE = 6,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OP_RETURN,
            1 => OpCode::OP_CONSTANT,
            2 => OpCode::OP_NEGATE,
            _ => panic!("Unknown"),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::OP_RETURN => 0,
            OpCode::OP_CONSTANT => 1,
            OpCode::OP_ADD => 2,
            OpCode::OP_SUBTRACT => 3,
            OpCode::OP_MULTIPLY => 4,
            OpCode::OP_DIVIDE => 5,
            OpCode::OP_NEGATE => 6,
        }
    }
}
