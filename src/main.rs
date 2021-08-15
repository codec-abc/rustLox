#[macro_use]
extern crate num_derive;

use std::{env, fs, io::{self, Write}, process::exit};

use chunk::{Chunk};
use vm::{InterpretResult, VM};

mod scanner;
mod compiler;
mod chunk;
mod value;
mod vm;
mod object;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        repl();
    } else if argc == 2 {
        run_file(&args[1]);
    } else {
        println!("Usage: rustLox [path]");
        exit(64);
    }

    // let mut chunk = Chunk::new();

    // chunk.write_chunk(map_opcode_to_binary(OpCode::OpConstant), 123);
    // let constant_index = chunk.add_constant(1.2);
    // chunk.write_chunk(constant_index as u8, 123);

    // chunk.write_chunk(map_opcode_to_binary(OpCode::OpConstant), 123);
    // let constant_index = chunk.add_constant(3.4);
    // chunk.write_chunk(constant_index as u8, 123);

    // chunk.write_chunk(map_opcode_to_binary(OpCode::OpAdd), 123);

    // let constant_index = chunk.add_constant(5.6);
    // chunk.write_chunk(map_opcode_to_binary(OpCode::OpConstant), 123);
    // chunk.write_chunk( constant_index as u8, 123);

    // chunk.write_chunk( map_opcode_to_binary(OpCode::OpDivide), 123);


    // chunk.write_chunk(map_opcode_to_binary(OpCode::OpNegate), 123);
    // chunk.write_chunk(map_opcode_to_binary(OpCode::OpReturn), 123);

    // chunk.disassemble_chunk("test chunk");

    // let mut vm = VM::new(chunk);

    // let result = vm.interpret();

    //println!("{:?}", result);
}

fn run_file(path: &str) {
    let read_result = fs::read_to_string(path);
    if read_result.is_err() {
        println!("Could not read file {} because {:?}", path, read_result.err().unwrap());
        exit(74);
    }
    let str = read_result.unwrap();
    let result: InterpretResult = interpret(&str);
    match result {
        InterpretResult::InterpretCompileError => {
            exit(65);
        }
        InterpretResult::InterpretRuntimeError => {
            exit(70);
        }
        _ => {}
    }
}

fn repl() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut s = String::new();
        let read_result = io::stdin().read_line(&mut s);
        if read_result.is_err() {
            println!("error reading line");
            break;
        } else {
            interpret(&s);
        }  
    }
}

fn interpret(code: &str)  -> InterpretResult {
    let chunk = Chunk::new();
    let mut vm = VM::new(chunk);
    vm.interpret(code)
}
