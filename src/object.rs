use generational_arena::Index;

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

impl Object {
    pub fn is_string(&self) -> bool {
        match &self {
            Self::ObjString(_) => { true }
        }
    }

    pub fn as_string(&self) -> &ObjectString {
        match &self {
            Self::ObjString(a) => { return a },
        }
    }
}