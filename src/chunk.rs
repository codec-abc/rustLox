use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

pub fn map_instruction_to_opcode(instruction: u8) -> OpCode {
    match instruction {
        0u8 => OpCode::OpReturn,
        1u8 => OpCode::OpConstant,
        2u8 => OpCode::OpNegate,
        3u8 => OpCode::OpAdd,
        4u8 => OpCode::OpSubtract,
        5u8 => OpCode::OpMultiply,
        6u8 => OpCode::OpDivide,
        _ => unreachable!()
    }
}

pub fn map_opcode_to_instruction(opcode: OpCode) -> u8 {
    match opcode {
        OpCode::OpReturn => 0u8,
        OpCode::OpConstant => 1u8,
        OpCode::OpNegate => 2u8,
        OpCode::OpAdd => 3u8,
        OpCode::OpSubtract => 4u8,
        OpCode::OpMultiply => 5u8,
        OpCode::OpDivide => 6u8,
        _ => unreachable!()
    }
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec!(),
            lines: vec!(),
            constants: vec!(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn free_chunk(&mut self) {
        self.code = vec!();
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }

    pub fn disassemble_chunk(&mut self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            let instruction = self.code[offset];
            offset = self.disassemble_instruction(instruction, offset);
        }
    }

    fn disassemble_instruction(&self, instruction: u8, offset: usize) -> usize {
        print!("{:#06x?} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{} ", self.lines[offset]);
        }

        let parsed_instruction = map_instruction_to_opcode(instruction);
        match parsed_instruction {
            OpCode::OpReturn => {
                println!("OpReturn");
                offset + 1
            }
            OpCode::OpConstant => {
                let constant = self.constants[self.code[offset + 1] as usize];
                println!("OpConstant {}", constant);
                offset + 2
            }
            OpCode::OpNegate => {
                println!("OpNegate");
                offset + 1
            }

            OpCode::OpAdd => {
                println!("OpAdd");
                offset + 1
            }
            OpCode::OpSubtract => {
                println!("OpSubtract");
                offset + 1
            }
            OpCode::OpMultiply => {
                println!("OpMultiply");
                offset + 1
            }
            OpCode::OpDivide => {
                println!("OpDivide");
                offset + 1
            }

            _ => {
                unimplemented!("disassemble_instruction, missing {:?}", parsed_instruction);
            }
        }
    }

}