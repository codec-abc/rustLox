use chunk::{Chunk, OpCode, map_opcode_to_instruction};

mod chunk;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpConstant), 123);
    let constant_index = chunk.add_constant(1.2f64);
    chunk.write_chunk(constant_index as u8, 123);
    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpReturn), 123);
}
