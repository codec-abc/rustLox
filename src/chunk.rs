use crate::value::Value;

enum OpCode {
    OP_CONSTANT(Value),
    OP_RETURN,
}

struct Chunk {
    // count: isize,
    // capacity: isize,
    code: Vec<u8>,
}

fn map_instruction_to_opcode(instruction: u8) -> OpCode {
    match instruction {
        0u8 => OpCode::OP_RETURN,
        _ => unreachable!()

    }

}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            // count: 0,
            // capacity: 0,
            code: vec!(),
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