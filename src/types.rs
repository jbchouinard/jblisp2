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
    } // Will blow the stack on circular list...
    pub fn is_list(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Pair(_, y) => match &**y {
                JValue::Cell(c) => c.is_list(),
                _ => false,
            },
        }
    }
}

#[derive(Clone)]
pub struct JBuiltin {
    pub name: String,
    pub f: Rc<dyn Fn(Vec<JValueRef>, JEnvRef) -> JResult>,
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
    SExpr(Vec<JValueRef>),
    Cell(JCell),
    Int(JTInt),
    Bool(bool),
    Symbol(String),
    String(String),
    Error(JError),
    Builtin(JBuiltin),
    BuiltinMacro(JBuiltin),
    Lambda(Box<JLambda>),
}

impl JValue {
    pub fn into_ref(self) -> JValueRef {
        Rc::new(self)
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

#[macro_export]
macro_rules! jsexpr {
    ($($args:expr),*) => {{
        let mut list = vec![];
        $(
            list.push($args);
        )*
        JValue::SExpr(list).into_ref()
    }}
}
