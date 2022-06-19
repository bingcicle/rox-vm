use crate::{chunk::Chunk, opcodes::OpCode, value::Value};

#[derive(Debug, PartialEq)]
enum InterpretResult {
    OK,
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
pub struct VirtualMachine {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    trace: bool,
}

impl VirtualMachine {
    pub fn new(chunk: Chunk, trace: bool) -> Self {
        Self {
            chunk,
            ip: 0,
            trace,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;

        return self.run();
    }

    fn run(&mut self) -> InterpretResult {
        let byte = self.read_byte();

        if self.trace {
            for slot in self.stack.iter() {
                println!("[ {} ]", slot);
            }
        }

        match OpCode::from(byte) {
            OpCode::OP_RETURN => {
                let popped = self.stack.pop().unwrap();
                println!("{:?}", popped);
                InterpretResult::OK
            }
            OpCode::OP_CONSTANT => {
                let byte = self.read_byte() as usize;
                let constant = self.chunk.read_constant(byte);
                self.stack.push(constant);
                InterpretResult::OK
            }
            OpCode::OP_ADD => self.binary_op('+'),
            OpCode::OP_SUBTRACT => self.binary_op('-'),
            OpCode::OP_MULTIPLY => self.binary_op('*'),
            OpCode::OP_DIVIDE => self.binary_op('/'),
            OpCode::OP_NEGATE => {
                let value = self.stack.pop().unwrap() * Value::Number(-1.0);
                self.stack.push(value);
                InterpretResult::OK
            }
        }
    }

    fn binary_op(&self, op: char) -> InterpretResult {
        InterpretResult::OK
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn smoke_test_vm() -> Result<()> {
        let chunk = Chunk::new();
        let _vm = VirtualMachine::new(chunk, false);
        Ok(())
    }

    #[test]
    fn test_op_return() {
        let mut chunk = Chunk::new();
        let index = chunk.add_constant(Value::Number(1.2));
        chunk.write(OpCode::OP_CONSTANT.into(), 123);
        chunk.write(index as u8, 123);
        chunk.write(OpCode::OP_RETURN.into(), 123);

        let mut vm = VirtualMachine::new(chunk, true);
        let res = vm.run();
        assert_eq!(res, InterpretResult::OK)
    }

    #[test]
    fn test_op_add() {
        let mut chunk = Chunk::new();
        let mut index = chunk.add_constant(Value::Number(3.4));
        chunk.write(OpCode::OP_CONSTANT.into(), 123);
        chunk.write(index as u8, 123);

        chunk.write(OpCode::OP_ADD.into(), 123);

        index = chunk.add_constant(Value::Number(5.6));
        chunk.write(OpCode::OP_CONSTANT.into(), 123);
        chunk.write(index as u8, 123);

        chunk.write(OpCode::OP_DIVIDE.into(), 123);
        chunk.write(OpCode::OP_NEGATE.into(), 123);

        chunk.write(OpCode::OP_RETURN.into(), 123);

        let mut vm = VirtualMachine::new(chunk, true);
        println!("{:?}", vm);
        let res = vm.run();
        assert_eq!(res, InterpretResult::OK)
    }
}
