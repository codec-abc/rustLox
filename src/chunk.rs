use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant,
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
        print!("{:#0x?} ", offset);
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
            _ => {
                unimplemented!("disassemble_instruction, missing {:?}", parsed_instruction);
            }
        }
    }

}