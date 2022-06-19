use crate::{opcodes::OpCode, value::Value};

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    constants: Vec<Value>,
    count: usize,
    lines: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![],
            count: 0,
        }
    }

    pub fn write(&mut self, byte: u8, line: u8) {
        self.code.push(byte);
        self.lines.push(line);
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn read_constant(&self, ip: usize) -> Value {
        self.constants[ip].clone()
    }

    pub fn disassemble(&mut self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;

        while offset < self.count {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&mut self, offset: usize) -> usize {
        let instruction = OpCode::from(self.code[offset]);
        print!("{} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  | ");
        } else {
            print!("{} ", self.lines[offset]);
        }
        match instruction {
            OpCode::OP_CONSTANT => constant_instruction("OP_CONSTANT", self, offset),
            OpCode::OP_ADD => simple_instruction("OP_ADD", offset),
            OpCode::OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
            OpCode::OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
            OpCode::OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
            OpCode::OP_NEGATE => simple_instruction("OP_NEGATE", offset),
            OpCode::OP_RETURN => simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode: {:?}", instruction);
                return offset + 1;
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &mut Chunk, offset: usize) -> usize {
    let index = chunk.code[offset + 1];
    let constant = &chunk.constants[index as usize];
    println!("{} {} {:?}", name, index, constant);
    offset + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_op_return() {
        let expected_chunk = Chunk {
            code: vec![OpCode::OP_RETURN.into()],
            constants: vec![],
            lines: vec![],
            count: 1,
        };

        let mut chunk = Chunk::new();
        chunk.write(OpCode::OP_RETURN.into(), 123);

        assert_eq!(chunk.disassemble_instruction(0), 1);
        assert_eq!(chunk.code, expected_chunk.code)
    }

    #[test]
    fn test_write_op_constant() {
        let expected_chunk = Chunk {
            code: vec![OpCode::OP_CONSTANT.into(), 0],
            constants: vec![],
            lines: vec![],
            count: 1,
        };

        let mut chunk = Chunk::new();
        let index = chunk.add_constant(Value::Number(1.2));
        chunk.write(OpCode::OP_CONSTANT.into(), 123);
        chunk.write(index as u8, 123);

        assert_eq!(chunk.disassemble_instruction(0), 2);
        assert_eq!(chunk.code, expected_chunk.code);
    }

    #[test]
    fn test_write_binary_ops() {
        let expected_chunk = Chunk {
            code: vec![
                OpCode::OP_CONSTANT.into(),
                0,
                OpCode::OP_ADD.into(),
                1,
                1,
                OpCode::OP_DIVIDE.into(),
                OpCode::OP_NEGATE.into(),
                OpCode::OP_RETURN.into(),
            ],
            constants: vec![],
            lines: vec![],
            count: 1,
        };

        let mut chunk = Chunk::new();
        let index = chunk.add_constant(Value::Number(3.4));
        chunk.write(OpCode::OP_CONSTANT.into(), 123);
        chunk.write(index as u8, 123);

        chunk.write(OpCode::OP_ADD.into(), 123);

        let index = chunk.add_constant(Value::Number(5.6));
        chunk.write(OpCode::OP_CONSTANT.into(), 123);
        chunk.write(index as u8, 123);

        chunk.write(OpCode::OP_DIVIDE.into(), 123);
        chunk.write(OpCode::OP_NEGATE.into(), 123);
        chunk.write(OpCode::OP_RETURN.into(), 123);

        assert_eq!(chunk.code, expected_chunk.code);
    }
}
