use crate::{chunk::{Chunk, OpCode, map_binary_to_opcode}, value::{Value, print_value}};

#[derive(Debug, Clone)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_MAX: usize = 256;
const DEBUG_TRACE_EXECUTION : bool = true;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

impl VM {

    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk: chunk,
            ip: 0,
            stack: [0.0f64; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();

        if !self.compile(source, &mut chunk) {
            return InterpretResult::InterpretCompileError;
        }

        self.chunk = chunk;
        self.stack_top = 0;
        self.ip = 0;
        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top = self.stack_top + 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top = self.stack_top - 1;
        self.stack[self.stack_top]
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_instruction();
            match instruction {
                OpCode::OpReturn => {
                    print_value(self.pop());
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let value = self.read_constant();
                    self.push(value);
                }
                OpCode::OpNegate => {
                    let value = self.pop();
                    self.push(-value);
                }
                op @ OpCode::OpAdd => {
                    self.binary_op(op);
                }
                op @ OpCode::OpSubtract => {
                    self.binary_op(op);
                }
                op @ OpCode::OpMultiply => {
                    self.binary_op(op);
                }
                op @ OpCode::OpDivide => {
                    self.binary_op(op);
                }

                _ =>  { 
                    panic!("OpCode not implemented {:?}", instruction); 
                }
            }
        }
    }

    fn binary_op(&mut self, opcode: OpCode) {
        let b = self.pop();
        let a = self.pop();

        let result = match opcode {
            OpCode::OpAdd => {
                a + b
            }
            OpCode::OpSubtract => {
                a - b
            }
            OpCode::OpMultiply => {
                a * b
            }
            OpCode::OpDivide => {
                a / b
            }
            _ => { 
                unimplemented!("binary op not implemented");
            }
        };

        self.push(result);
    }

    fn read_instruction(&mut self) -> OpCode {
        let byte = self.get_next_byte();
        map_binary_to_opcode(byte)
    }

    fn get_next_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip = self.ip + 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.get_next_byte();
        self.chunk.constants[byte as usize]
    }
}