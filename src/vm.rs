use std::{collections::HashMap};
use generational_arena::{Arena, Index};

use crate::{chunk::{Chunk, OpCode, map_binary_to_opcode}, compiler::Parser, object::{Object, ObjectString}, value::{Value, print_value, values_equal}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_MAX: usize = 256;
const INIT: Value = Value::Nil;

struct VMString {
    pub strings: Arena<String>,
    pub string_to_string_index: HashMap<String, Index>,
    pub string_to_string_obj: HashMap<String, Index>
}

impl VMString {
    pub fn new() -> VMString {
        VMString {
            strings: Arena::new(),
            string_to_string_index: HashMap::new(),
            string_to_string_obj: HashMap::new(),
        }
    }
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
    objects: Arena<Object>,
    globals: HashMap<String, Value>,
    strings: VMString
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
            objects: Arena::new(),
            globals: HashMap::new(),
            strings: VMString::new(),
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
                    print_value(self.pop(), &self);
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
                OpCode::OpSetGlobal => {
                    let name = self.read_string();
                    let value = self.peek(0);
                    if self.globals.contains_key(&name) {
                        let existing_value = self.globals.get_mut(&name);
                        *existing_value.unwrap() = value;
                    } else {
                        self.globals.insert(name, value);
                    }
                }
                OpCode::OpGetLocal => {
                    let slot = self.get_next_byte();
                    let to_push = self.stack[slot as usize].clone();
                    self.push(to_push);
                }
                OpCode::OpSetLocal => {
                    let slot = self.get_next_byte();
                    self.stack[slot as usize] = self.peek(0);
                }

            }
        }
    }

    fn read_string(&mut self) -> String {
        let constant = self.read_constant();
        let obj = constant.as_object();
        let name = obj.as_string();
        self.strings.strings.get(*name.id()).unwrap().clone()
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
        
        c.push_str(self.get_string_from_index(&a_str.id()));
        c.push_str(self.get_string_from_index(&b_str.id()));

        //let id = self.get_or_create_string(&c);

        //let object = Object::ObjString(ObjectString::new(id));
        let object = self.get_or_create_string_object(&c);
        self.push(object);
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

    // READ_BYTE
    fn get_next_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip = self.ip + 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.get_next_byte();
        self.chunk.constants[byte as usize].clone()
    }

    fn get_string_from_index(&self, index: &Index) -> &String {
        self.strings.strings.get(*index).unwrap()
    }

    fn get_index_from_string(&self, string: &str) -> Option<&Index> {
        self.strings.string_to_string_index.get(string)
    }

    fn create_new_string(&mut self, string: &str) -> Index {
        if self.strings.string_to_string_index.contains_key(string) {
            panic!("Avoid duplication of strings");
        }

        let id = self.strings.strings.insert(string.into());
        self.strings.string_to_string_index.insert(string.into(), id);
        id
    }

    fn get_or_create_string(&mut self, string: &str) -> (Index, bool) {

        let mut created = false;
        let id = self.get_index_from_string(string);

        let id = if id.is_some() {
            id.unwrap().clone()
        } else {
            let id = self.create_new_string(string);
            created = true;
            id
        };

        (id, created)
    }

    fn create_new_obj_with_existing_string(&mut self, string_index: Index) -> Index {
        let object = Object::ObjString(ObjectString::new(string_index));
        let object_index = self.objects.insert(object);
        object_index
    }

    pub fn get_or_create_string_object(&mut self, string: &str) -> Value { //::Object(id, obj)
        let (string_id, was_string_created) = self.get_or_create_string(string);

        let obj_index = 
            if was_string_created {
                self.create_new_obj_with_existing_string(string_id)
            } else {
                let id = self.strings.string_to_string_obj.get(string);
                id.unwrap().clone()
            };

        if was_string_created {
            self.strings.string_to_string_obj.insert(string.into(), obj_index.clone());
        }

        let obj_string = ObjectString::new(string_id.clone());
        let obj = Object::ObjString(obj_string);
        Value::Object(obj_index, obj)
    }

    pub fn remove_string(&mut self, string: &str) {
        let id = self.strings.string_to_string_index.get(string.into()).unwrap();
        self.strings.strings.remove(*id);
        self.strings.string_to_string_obj.remove_entry(string);

        let _ = self.strings.string_to_string_index.remove_entry(string.into());
    }

    pub fn print_object(&self, _: &Index, o: &Object) {
        match o {
            Object::ObjString(a) => println!("{}", self.get_string_from_index(a.id())) ,
        }
    }

    pub fn dump_stats(&mut self) {
        println!("================================================");
        println!("VM contains {} objects", self.objects.len());
        for (_, object) in self.objects.iter() {
            print!("object is {:?}. ", object);
            match &object {
                Object::ObjString(str_obj) => {
                    let id = str_obj.id();
                    let str = self.strings.strings.get(*id).unwrap();
                    println!("In particular, object is a string: {}", str); 
                }
            }
        }
        println!("================================================");
        println!("VM contains {} strings", self.strings.strings.len());
        for (_, string) in self.strings.strings.iter() {
            println!("VM String: {}", string);
        }

        println!("================================================");
        println!("VM contains {} globals", self.globals.len());
        for (key, value) in self.globals.iter() {
            println!("object {:?} has key {:?}", key, value);
        }
        println!("================================================");


        self.chunk.disassemble_chunk();
    }
}
