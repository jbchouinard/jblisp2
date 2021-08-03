use std::fmt;
use std::rc::Rc;

use crate::env::JEnvRef;
use crate::*;

pub type JTInt = i128;

#[derive(Debug, PartialEq, Clone)]
pub enum JCell {
    Nil,
    Pair(JValueRef, JValueRef),
}

impl JCell {
    pub fn cons(x: JValueRef, y: JValueRef) -> Self {
        Self::Pair(x, y)
    }
    pub fn car(&self) -> JResult {
        match self {
            Self::Nil => Err(JError::new("ValueError", "cannot call car on nil")),
            Self::Pair(x, _) => Ok(Rc::clone(x)),
        }
    }
    pub fn cdr(&self) -> JResult {
        match self {
            Self::Nil => Err(JError::new("ValueError", "cannot call cdr on nil")),
            Self::Pair(_, y) => Ok(Rc::clone(y)),
        }
    }
    // Will blow the stack on circular list...
    pub fn is_list(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Pair(_, y) => match &**y {
                JValue::Cell(c) => c.is_list(),
                _ => false,
            },
        }
    }
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }
    pub fn iter(&self) -> Result<JCellIterator, JError> {
        if !self.is_list() {
            return Err(JError::new("ValueError", "cannot iter a non-list"));
        }
        Ok(JCellIterator { head: self })
    }
}

pub struct JCellIterator<'a> {
    head: &'a JCell,
}

impl Iterator for JCellIterator<'_> {
    type Item = JValueRef;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.head {
            JCell::Nil => None,
            JCell::Pair(x, y) => {
                match &**y {
                    JValue::Cell(c) => self.head = c,
                    _ => self.head = &JCell::Nil,
                };
                Some(Rc::clone(x))
            }
        }
    }
}

pub fn vec_to_list(mut v: Vec<JValueRef>) -> JCell {
    let mut cur = JCell::Nil;
    v.reverse();
    for val in v {
        cur = JCell::cons(val, JValue::Cell(cur).into_ref());
    }
    cur
}

#[derive(Clone)]
pub struct JBuiltin {
    pub name: String,
    pub f: Rc<dyn Fn(&JCell, JEnvRef) -> JResult>,
}

impl fmt::Debug for JBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "builtin {}", self.name)
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
    pub code: JValueRef,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JValue {
    Cell(JCell),
    Quoted(JValueRef),
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

impl fmt::Display for JValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", jrepr(self))
    }
}

impl JValue {
    pub fn into_ref(self) -> JValueRef {
        Rc::new(self)
    }
    pub fn into_quoted(self) -> JValue {
        JValue::Quoted(self.into_ref())
    }
    pub fn to_int(&self) -> Result<JTInt, JError> {
        match self {
            Self::Int(n) => Ok(*n),
            _ => Err(JError::new("TypeError", "expected an int")),
        }
    }
}

pub type JValueRef = Rc<JValue>;

pub fn jint(n: JTInt) -> JValueRef {
    JValue::Int(n).into_ref()
}

pub fn jsym(s: &str) -> JValueRef {
    JValue::Symbol(s.to_string()).into_ref()
}

pub fn jstr(s: &str) -> JValueRef {
    JValue::String(s.to_string()).into_ref()
}

pub fn jquote(v: JValueRef) -> JValueRef {
    JValue::Quoted(Rc::clone(&v)).into_ref()
}

#[macro_export]
macro_rules! jsexpr {
    ($($args:expr),*) => {{
        let mut list = vec![];
        $(
            list.push($args);
        )*
        JValue::Cell(vec_to_list(list)).into_ref()
    }}
}
