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
            return Err(JError::new("ValueError", "cannot iter a non-list"));
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

impl fmt::Debug for JBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for JBuiltin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct JLambda {
    pub closure: JEnvRef,
    pub params: Vec<String>,
    pub code: JValRef,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JVal {
    Nil,
    Pair(JPair),
    Quoted(JValRef),
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
        write!(f, "{}", jrepr(self))
    }
}

impl JVal {
    pub fn into_ref(self) -> JValRef {
        Rc::new(self)
    }
    pub fn to_int(&self) -> Result<JTInt, JError> {
        match self {
            Self::Int(n) => Ok(*n),
            _ => Err(JError::new("TypeError", "expected an int")),
        }
    }
    pub fn from_error(je: JError) -> JValRef {
        JVal::Error(je).into_ref()
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
            _ => Err(JError::new("TypeError", "not a list")),
        }
    }

    // Constructors
    pub fn quote(v: JValRef) -> JValRef {
        JVal::Quoted(v).into_ref()
    }
    pub fn list(mut v: Vec<JValRef>) -> JValRef {
        let mut cur = Self::nil();
        v.reverse();
        for val in v {
            cur = Self::cons(val, cur);
        }
        cur
    }
    pub fn cons(left: JValRef, right: JValRef) -> JValRef {
        JVal::Pair(JPair::cons(left, right)).into_ref()
    }
    pub fn nil() -> JValRef {
        JVal::Nil.into_ref()
    }
    pub fn _int(n: JTInt) -> JValRef {
        JVal::Int(n).into_ref()
    }
    pub fn int(n: JTInt, state: &mut JState) -> JValRef {
        state.make_int(n)
    }
    pub fn bool(p: bool, state: &JState) -> JValRef {
        state.get_bool(p)
    }
    pub fn _sym(s: String) -> JValRef {
        JVal::Symbol(s).into_ref()
    }
    pub fn sym(s: String, state: &mut JState) -> JValRef {
        state.make_sym(s)
    }
    pub fn _str(s: String) -> JValRef {
        JVal::String(s).into_ref()
    }
    pub fn str(s: String, state: &mut JState) -> JValRef {
        state.make_str(s)
    }
    pub fn err(etype: &str, emsg: &str) -> JValRef {
        JVal::Error(JError::new(etype, emsg)).into_ref()
    }
    pub fn lambda(clos: JEnvRef, params: Vec<String>, code: JValRef) -> JValRef {
        JVal::Lambda(Box::new(JLambda {
            closure: clos,
            params,
            code,
        }))
        .into_ref()
    }
    pub fn lmacro(clos: JEnvRef, params: Vec<String>, code: JValRef) -> JValRef {
        JVal::Macro(Box::new(JLambda {
            closure: clos,
            params,
            code,
        }))
        .into_ref()
    }
}
