use crate::value::Value;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum OpCode {
    OpConstant = 0,
    OpNil = 1,
    OpTrue = 2,
    OpFalse = 3,
    OpEqual = 4,
    OpGreater = 5,
    OpLess = 6,
    OpAdd = 7,
    OpSubtract = 8,
    OpMultiply = 9,
    OpDivide = 10,
    OpNot = 11,
    OpNegate = 12,
    OpReturn = 13,
    OpPrint = 14,
    OpPop = 15,
    OpDefineGlobal = 16,
    OpSetGlobal = 17,
    OpGetGlobal = 18,
    OpGetLocal = 19,
    OpSetLocal = 20,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

pub fn map_binary_to_opcode(instruction: u8) -> OpCode {
    num::FromPrimitive::from_u8(instruction).unwrap()
}

pub fn map_opcode_to_binary(opcode: OpCode) -> u8 {
    num::ToPrimitive::to_u8(&opcode).unwrap()
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

    // pub fn free_chunk(&mut self) {
    //     self.code = vec!();
    // }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }

    pub fn disassemble_chunk(&mut self) {
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

        let parsed_instruction = map_binary_to_opcode(instruction);
        match parsed_instruction {
            OpCode::OpReturn => {
                println!("OpReturn");
                offset + 1
            }
            OpCode::OpConstant => {
                let constant = self.constants[self.code[offset + 1] as usize].clone();
                println!("OpConstant {:?}", constant);
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
            OpCode::OpNil => {
                println!("OpNil");
                offset + 1
            }
            OpCode::OpTrue => {
                println!("OpTrue");
                offset + 1
            }
            OpCode::OpFalse => {
                println!("OpFalse");
                offset + 1
            }
            OpCode::OpNot => {
                println!("OpNot");
                offset + 1
            }
            OpCode::OpEqual => {
                println!("OpEqual");
                offset + 1
            }
            OpCode::OpGreater => {
                println!("OpGreater");
                offset + 1
            }
            OpCode::OpLess => {
                println!("OpLess");
                offset + 1
            }
            OpCode::OpPrint => {
                println!("OpPrint");
                offset + 1
            }
            OpCode::OpPop => {
                println!("OpPop");
                offset + 1
            }
            OpCode::OpDefineGlobal => {
                println!("OpDefineGlobal");
                offset + 1
            }
            OpCode::OpGetGlobal => {
                println!("OpGetGlobal");
                offset + 1
            }
            OpCode::OpSetGlobal => {
                println!("OpSetGlobal");
                offset + 1
            }
            OpCode::OpSetLocal => {
                println!("OpSetLocal");
                offset + 1
            }
            OpCode::OpGetLocal => {
                println!("OpGetLocal");
                offset + 1
            }
        }
    }

}