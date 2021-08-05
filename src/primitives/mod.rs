use std::fmt;
use std::rc::Rc;

use crate::env::JEnvRef;
use crate::state::JState;
use crate::*;

pub mod intern;

pub type JTInt = i128;

#[derive(Debug, PartialEq, Clone)]
pub struct JPair(JValRef, JValRef);

impl JPair {
    pub fn cons(x: JValRef, y: JValRef) -> Self {
        Self(x, y)
    }
    pub fn car(&self) -> JValRef {
        Rc::clone(&self.0)
    }
    pub fn cdr(&self) -> JValRef {
        Rc::clone(&self.1)
    }
    // Will blow the stack on circular list...
    pub fn is_list(&self) -> bool {
        match &*self.1 {
            JVal::Nil => true,
            JVal::Pair(c) => c.is_list(),
            _ => false,
        }
    }
    pub fn iter(&self) -> Result<JListIterator, JError> {
        if !self.is_list() {
            return Err(JError::TypeError("can only iter lists".to_string()));
        }
        Ok(JListIterator { head: Some(self) })
    }
}

pub struct JListIterator<'a> {
    head: Option<&'a JPair>,
}

impl Iterator for JListIterator<'_> {
    type Item = JValRef;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.head {
            None => None,
            Some(JPair(x, y)) => {
                match &**y {
                    JVal::Pair(c) => self.head = Some(c),
                    _ => self.head = None,
                };
                Some(Rc::clone(x))
            }
        }
    }
}

#[derive(Clone)]
pub struct JBuiltin {
    pub name: String,
    pub f: Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>,
}

impl fmt::Display for JBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "builtin {}", &self.name)
    }
}

impl fmt::Debug for JBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("JBuiltin")
            .field("name", &self.name)
            .field("f", &format_args!("<Fn {:p}>", Rc::as_ptr(&self.f)))
            .finish()
    }
}

impl PartialEq for JBuiltin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(PartialEq, Clone)]
pub struct JLambda {
    pub closure: JEnvRef,
    pub params: Vec<String>,
    pub code: JValRef,
}

// Implement manually because of cyclical references from the closure's parent Envs
// back to the JVal
impl fmt::Debug for JLambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("JLambda")
            .field("closure", &format_args!("<JEnvRef {:p}>", &self.closure))
            .field("params", &self.params)
            .field("code", &self.code)
            .finish()
    }
}

// JVal's should only be constructed by JState, which manages interned values
#[derive(Debug, PartialEq, Clone)]
pub enum JVal {
    Nil,
    Pair(JPair),
    Quote(JValRef),
    Int(JTInt),
    Bool(bool),
    Symbol(String),
    String(String),
    Error(JError),
    Lambda(Box<JLambda>),
    Macro(Box<JLambda>),
    Builtin(JBuiltin),
    SpecialForm(JBuiltin),
}

pub type JValRef = Rc<JVal>;

impl fmt::Display for JVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", repr(self))
    }
}

impl JVal {
    pub fn into_ref(self) -> JValRef {
        Rc::new(self)
    }
    pub fn to_int(&self) -> Result<JTInt, JError> {
        match self {
            Self::Int(n) => Ok(*n),
            _ => Err(JError::TypeError("expected an int".to_string())),
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            JVal::Nil => true,
            JVal::Pair(p) => p.is_list(),
            _ => false,
        }
    }
    pub fn iter_list(&self) -> Result<JListIterator, JError> {
        match self {
            JVal::Nil => Ok(JListIterator { head: None }),
            JVal::Pair(p) => p.iter(),
            _ => Err(JError::TypeError("can only iter lists".to_string())),
        }
    }
}
