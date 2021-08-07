use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

pub struct Chunk {
    // count: isize,
    // capacity: isize,
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
}

pub fn map_instruction_to_opcode(instruction: u8) -> OpCode {
    match instruction {
        0u8 => OpCode::OpReturn,
        1u8 => OpCode::OpConstant,
        _ => unreachable!()
    }
}

pub fn map_opcode_to_instruction(opcode: OpCode) -> u8 {
    match opcode {
        OpCode::OpReturn => 0u8,
        OpCode::OpConstant => 1u8,
        _ => unreachable!()
    }
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            // count: 0,
            // capacity: 0,
            code: vec!(),
            lines: vec!(),
        }
    }

    fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }

    fn free_chunk(&mut self) {
        self.code = vec!();
    }

    // fn disassemble_chunk(&mut self, name: &str) {
    //     println!("== {} ==", name);
    //     for i in 0..self.code.len() {
    //         let instruction = self.code[i];
    //     }
    // }

}