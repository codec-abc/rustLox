use std::{collections::HashMap, rc::Rc};

use crate::{chunk::{Chunk, OpCode, map_binary_to_opcode}, compiler::Parser, object::{Object, ObjectString}, value::{Value, print_value, values_equal}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_MAX: usize = 256;
const INIT: Value = Value::Nil;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
    objects: Vec<Rc<Object>>,
    globals: HashMap<String, Value>,
}

fn is_falsey(value: Value) -> bool {
    value.is_nil() || (value.is_bool() && !value.as_bool())
}

impl VM {

    pub fn new(chunk: Chunk) -> VM {
        let array = [INIT; STACK_MAX];
        VM {
            chunk: chunk,
            ip: 0,
            stack: array,
            stack_top: 0,
            objects: vec!(),
            globals: HashMap::new(),
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        
        let mut parser = Parser::new(source);

        if !parser.compile( self) {
            return InterpretResult::InterpretCompileError;
        }

        self.chunk = parser.get_compiling_chunk();
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
        self.stack[self.stack_top].clone()
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack_top -1 - distance].clone()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_instruction();
            match instruction {
                OpCode::OpReturn => {
                    //print_value(self.pop());
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let value = self.read_constant();
                    self.push(value);
                }
                OpCode::OpNot => {
                    let popped = self.pop();
                    self.push(Value::Boolean(is_falsey(popped)))
                }
                OpCode::OpNegate => {
                    if ! self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let value = self.pop();
                    self.push(Value::Number(-value.as_number()));
                }
                op @ OpCode::OpAdd => {
                    let result = self.binary_op(op);
                    if result != InterpretResult::InterpretOk {
                        return result;
                    }
                }
                op @ OpCode::OpSubtract => {
                    let result = self.binary_op(op);
                    if result != InterpretResult::InterpretOk {
                        return result;
                    }
                }
                op @ OpCode::OpMultiply => {
                    let result = self.binary_op(op);
                    if result != InterpretResult::InterpretOk {
                        return result;
                    }
                }
                op @ OpCode::OpDivide => {
                    let result = self.binary_op(op);
                    if result != InterpretResult::InterpretOk {
                        return result;
                    }
                }
                OpCode::OpNil => {
                    self.push(Value::Nil);
                }
                OpCode::OpTrue => {
                    self.push(Value::Boolean(true));
                }
                OpCode::OpFalse => {
                    self.push(Value::Boolean(false));
                }
                OpCode::OpEqual => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(Value::Boolean(values_equal(a, b)));
                }
                op @ OpCode::OpGreater => {
                    let result = self.binary_op(op);
                    if result != InterpretResult::InterpretOk {
                        return result;
                    }
                }
                op @ OpCode::OpLess => {
                    let result = self.binary_op(op);
                    if result != InterpretResult::InterpretOk {
                        return result;
                    }
                }
                OpCode::OpPrint => {
                    print_value(self.pop());
                    println!("");
                }
                OpCode::OpPop => {
                    self.pop();
                }
                OpCode::OpDefineGlobal => {
                    let name = self.read_string();
                    let value = self.peek(0);
                    self.globals.insert(name, value);
                    self.pop();
                }
                OpCode::OpGetGlobal => {
                    let name = self.read_string();
                    let maybe_key = self.globals.get(&name);
                    if maybe_key.is_none() {
                        let message = format!("Undefined variable '{}'", &name);
                        self.runtime_error(&message);
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let key = maybe_key.unwrap().clone();
                    self.push(key);
                }

            }
        }
    }

    fn read_string(&mut self) -> String {
        let constant = self.read_constant();
        let obj = constant.as_object();
        let name = obj.as_string();
        name.str().into()
    }

    fn runtime_error(&mut self, message: &str) {
        println!("{}", message);

        self.reset_stack();
    }

    fn concatenate(&mut self) {
        let b = self.pop();
        let a = self.pop();

        let b_obj = b.as_object();
        let a_obj = a.as_object();

        let b_str = b_obj.as_string();
        let a_str = a_obj.as_string();

        let mut c = String::new();
        c.push_str(a_str.str());
        c.push_str(b_str.str());

        let str_obj = ObjectString::new(&c);
        let object = Rc::new(Object::ObjString(str_obj));
        self.add_object(object.clone());

        self.push(Value::Object(object));
    }

    pub fn add_object(&mut self, object: Rc<Object>) {
        self.objects.push(object);
    }

    fn binary_op(&mut self, opcode: OpCode) -> InterpretResult {

        if self.peek(0).is_string() && self.peek(1).is_string() {
            self.concatenate();
            return InterpretResult::InterpretOk;
        }

        if !self.peek(0).is_number() || !self.peek(1).is_number() {
            self.runtime_error("Operands must be numbers.");
            return InterpretResult::InterpretRuntimeError;
        }

        let b = self.pop().as_number();
        let a = self.pop().as_number();

        let result = match opcode {
            OpCode::OpAdd => {
                Value::Number(a + b)
            }
            OpCode::OpSubtract => {
                Value::Number(a - b)
            }
            OpCode::OpMultiply => {
                Value::Number(a * b)
            }
            OpCode::OpDivide => {
                Value::Number(a / b)
            }
            OpCode::OpGreater => {
                Value::Boolean(a > b)
            }
            OpCode::OpLess => {
                Value::Boolean(a < b)
            }
            _ => { 
                unimplemented!("binary op not implemented");
            }
        };

        self.push(result);

        InterpretResult::InterpretOk
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
        self.chunk.constants[byte as usize].clone()
    }
}