use generational_arena::Index;

use crate::{object::{Object, print_object}, vm::VM};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Object(Index, Object), //object id
    Nil,
}

pub fn values_equal(a: Value, b: Value) -> bool {
    return a == b
}

impl Value {
    pub fn is_bool(&self) -> bool {
        match &self {
            Self::Boolean(_) => { true }
            _ => { false }
        }
    }

    pub fn is_nil(&self) -> bool {
        match &self {
            Self::Nil => { true }
            _ => { false }
        }
    }

    pub fn is_number(&self) -> bool {
        match &self {
            Self::Number(_) => { true }
            _ => { false }
        }
    }

    pub fn is_object(&self) -> bool {
        match &self {
            Self::Object(_, _) => { true }
            _ => { false }
        }
    }

    pub fn is_string(&self) -> bool {
        match &self {
            Self::Object(_, a) => { a.is_string() }
            _ => { false }
        }
    }

    pub fn as_object(&self) -> Object {
        match self {
            Self::Object(_, a) => { return a.clone() },
            _ => panic!("try to cast a non object value"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match &self {
            Self::Boolean(a) => { return *a },
            _ => panic!("try to cast a non bool value"),
        }
    }

    pub fn as_number(&self) -> f64 {
        match &self {
            Self::Number(a) => { return *a },
            _ => panic!("try to cast a non number value"),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}

pub fn print_value(value: Value, vm: &VM) {
    match value {
        Value::Boolean(b) => println!("{}", b),
        Value::Nil => println!("nil"),
        Value::Number(n) => println!("{}", n),
        Value::Object(_, o) => print_object(&o, vm),
    }
}
