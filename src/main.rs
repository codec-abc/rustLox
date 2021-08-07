use chunk::{Chunk, OpCode, map_opcode_to_instruction};
use vm::VM;

mod chunk;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpConstant), 123);
    let constant_index = chunk.add_constant(1.2);
    chunk.write_chunk(constant_index as u8, 123);

    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpConstant), 123);
    let constant_index = chunk.add_constant(3.4);
    chunk.write_chunk(constant_index as u8, 123);

    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpAdd), 123);

    let constant_index = chunk.add_constant(5.6);
    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpConstant), 123);
    chunk.write_chunk( constant_index as u8, 123);

    chunk.write_chunk( map_opcode_to_instruction(OpCode::OpDivide), 123);


    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpNegate), 123);
    chunk.write_chunk(map_opcode_to_instruction(OpCode::OpReturn), 123);

    chunk.disassemble_chunk("test chunk");

    let mut vm = VM::new(chunk);

    let result = vm.interpret();

    println!("{:?}", result);
}
