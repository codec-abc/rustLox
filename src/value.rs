use std::{fmt::Display, rc::Rc};

use crate::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Object(Rc<Object>),
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
            Self::Object(_) => { true }
            _ => { false }
        }
    }

    pub fn is_string(&self) -> bool {
        match &self {
            Self::Object(a) => { a.is_string() }
            _ => { false }
        }
    }

    pub fn as_object(&self) -> &Object {
        match &self {
            Self::Object(a) => { return a },
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Nil => write!(f, "Nil"),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Object(o) => write!(f, "{}", o),
        }
    }
}

pub fn print_value(value: Value) {
    println!("{:?}", value);
}
