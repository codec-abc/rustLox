use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ObjectString { 
    string: Rc<Box<String>>,
    //hash: u32,
}

impl ObjectString {
    pub fn new(str: &str) -> ObjectString {
        //let hash = hash_string(str);
        ObjectString {
            //hash: hash,
            string: Rc::new(Box::new(str.into()))
        }
    }

    pub fn str(&self) -> &str {
        &*self.string
    }

    // pub fn hash(&self) -> u32 {
    //     self.hash
    // }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    ObjString(ObjectString),
}

// pub fn hash_string(key: &str) -> u32 {
//     let mut hash = 2166136261u32;
//     let key_bytes = key.as_bytes();
//     for i in 0..key_bytes.len() {
//         let current_byte = key_bytes[i as usize];
//         hash = hash ^ (current_byte as u32);
//         hash = hash * 16777619;
//     }

//     hash
// }

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