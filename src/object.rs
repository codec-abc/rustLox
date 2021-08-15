use std::rc::Rc;

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum ObjectType {
//     ObjString
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ObjectString { 
    pub string: Rc<Box<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    ObjString(ObjectString),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Object::ObjString(a) => write!(f, "this object is a string whose value is {}", a.string),
        }
    }
}

impl Object {
    pub fn is_string(&self) -> bool {
        match &self {
            Self::ObjString(_) => { true }
            _ => { false }
        }
    }

    pub fn as_string(&self) -> &ObjectString {
        match &self {
            Self::ObjString(a) => { return a },
            _ => panic!("try to cast a non string obj"),
        }
    }
}