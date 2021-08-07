use crate::{chunk::{Chunk, OpCode, map_instruction_to_opcode}, value::{Value, print_value}};

#[derive(Debug, Clone)]
enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
}

impl VM {
    fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_instruction();
            match instruction {
                OpCode::OpReturn => {
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let value = self.read_constant();
                    print_value(value);
                }
                _ =>  { 
                    panic!("OpCode not implemented {:?}", instruction); 
                }
            }
        }
    }

    fn read_instruction(&mut self) -> OpCode {
        let byte = self.get_next_byte();
        map_instruction_to_opcode(byte)
    }

    fn get_next_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip = self.ip + 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.get_next_byte();
        unimplemented!("read constant");
    }
}