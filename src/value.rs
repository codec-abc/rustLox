use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
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

    pub fn as_bool(&self) -> bool {
        match &self {
            Self::Boolean(a) => { return *a },
            _ => panic!("try to cast a non bool value to a bool one"),
        }
    }

    pub fn as_number(&self) -> f64 {
        match &self {
            Self::Number(a) => { return *a },
            _ => panic!("try to cast a non number value to a number one"),
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
            Self::Number(n) => write!(f, "{}", n)
        }
    }
}

pub fn print_value(value: Value) {
    println!("{:?}", value);
}
