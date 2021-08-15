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
    let result = vm.interpret(code);
    vm.dump_stats();
    result
}
