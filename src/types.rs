use std::fmt;
use std::rc::Rc;

use crate::*;

pub type JTInt = i128;

#[derive(Clone)]
pub struct JBuiltin {
    pub name: String,
    pub f: Rc<dyn Fn(Vec<JValue>, &mut JEnv) -> JResult>,
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
    pub closure: JEnv,
    pub params: Vec<String>,
    pub code: JValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JValue {
    SExpr(Vec<JValue>),
    Int(JTInt),
    Symbol(String),
    Error(JError),
    Builtin(JBuiltin),
    BuiltinMacro(JBuiltin),
    Lambda(Box<JLambda>),
}

impl JValue {
    pub fn into_int(self) -> Result<JTInt, JError> {
        match self {
            Self::Int(n) => Ok(n),
            _ => Err(JError::new("TypeError", "expected an int")),
        }
    }
}

pub fn jint(n: JTInt) -> JValue {
    JValue::Int(n)
}

pub fn jsym(s: &str) -> JValue {
    JValue::Symbol(s.to_string())
}

#[macro_export]
macro_rules! jsexpr {
    ($($args:expr),*) => {{
        let mut list = vec![];
        $(
            list.push($args);
        )*
        JValue::SExpr(list)
    }}
}
