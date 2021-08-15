use generational_arena::Index;

use crate::vm::VM;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ObjectString { 
    id: Index,
}

impl ObjectString {
    pub fn new(index: Index) -> ObjectString {
        ObjectString {
            id: index
        }
    }

    pub fn id(&self) -> &Index {
        &self.id
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    ObjString(ObjectString),
}

pub fn print_object(object: &Object, vm: &VM) {
    match &object {
        Object::ObjString(a) => println!("{}", vm.get_string_from_index(a.id())) ,
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