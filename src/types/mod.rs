use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::env::JEnvRef;
use crate::state::JState;
use crate::*;

pub mod intern;

pub type JTInt = i128;
pub type JTFloat = f64;

static BUILTIN_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn builtin_id() -> usize {
    BUILTIN_COUNTER.fetch_add(1, Ordering::SeqCst)
}

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
            return Err(JError::new(TypeError, "can only iter lists"));
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

pub type JBuiltinFn = Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>;

#[derive(Clone)]
pub struct JBuiltin {
    id: usize,
    pub name: String,
    pub f: JBuiltinFn,
}

impl JBuiltin {
    pub fn new(name: String, f: JBuiltinFn) -> Self {
        Self {
            id: builtin_id(),
            name,
            f,
        }
    }
}

impl fmt::Display for JBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "builtin<{}> \"{}\"", self.id, &self.name)
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
        self.id == other.id
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JParams {
    Fixed(Vec<String>),
    Variadic(Vec<String>, String),
}

impl fmt::Display for JParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Fixed(params) => format!("({})", params.join(" ")),
                Self::Variadic(params, rest) => format!("({} . {})", params.join(" "), rest),
            }
        )
    }
}

impl JParams {
    pub fn new(mut names: Vec<String>) -> Result<Self, JError> {
        let np = names.len();
        let rest = if np > 1 && names[np - 2] == "." {
            let end = names.split_off(np - 2);
            Some(end[1].to_string())
        } else {
            None
        };
        for p in &names {
            if p == "." {
                return Err(JError::new(EvalError, "ill-formed params"));
            }
        }
        Ok(match rest {
            Some(rname) => Self::Variadic(names, rname),
            None => Self::Fixed(names),
        })
    }
    fn nargs_short(&self) -> String {
        match self {
            Self::Variadic(params, _) => format!("{}+", params.len()),
            Self::Fixed(params) => format!("{}", params.len()),
        }
    }
    fn nargs_long(&self) -> String {
        match self {
            Self::Variadic(params, _) => format!("at least {}", params.len()),
            Self::Fixed(params) => format!("{}", params.len()),
        }
    }
    pub fn bind(&self, args: JValRef, env: JEnvRef) -> Result<(), JError> {
        let params = match self {
            Self::Fixed(params) => params,
            Self::Variadic(params, _) => params,
        };
        let mut head = args;
        for p in params.iter() {
            let pair: &JPair = head.to_pair().map_err(|_| {
                JError::new(
                    ApplyError,
                    &format!("expected {} argument(s)", self.nargs_long()),
                )
            })?;
            env.define(p, pair.car());
            head = pair.cdr();
        }
        match self {
            Self::Variadic(_, p) => env.define(p, head),
            Self::Fixed(_) => match &*head {
                JVal::Nil => (),
                _ => {
                    return Err(JError::new(
                        ApplyError,
                        &format!("expected {} argument(s)", self.nargs_long()),
                    ))
                }
            },
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone)]
pub struct JLambda {
    pub closure: JEnvRef,
    pub params: JParams,
    pub code: Vec<JValRef>,
    pub defpos: Option<PositionTag>,
    pub name: Option<String>,
}

impl fmt::Display for JLambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "({}) \"{}\"", self.params.nargs_short(), name),
            None => write!(f, "({})", self.params.nargs_short()),
        }
    }
}

// Implement manually because of cyclical references from the closure's parent Envs
// back to the JVal
impl fmt::Debug for JLambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JLambda")
            .field("closure", &format_args!("<JEnvRef {:p}>", &self.closure))
            .field("params", &self.params)
            .field("code", &self.code)
            .finish()
    }
}

pub type JVector = RefCell<Vec<JValRef>>;

// JVal's should only be constructed by JState, which manages interned values
#[derive(Debug, PartialEq, Clone)]
pub enum JVal {
    Int(JTInt),
    Float(JTFloat),
    Bool(bool),
    Symbol(String),
    String(String),
    Vector(JVector),
    Nil,
    Pair(JPair),
    Error(JError),
    Quote(JValRef),
    Quasiquote(JValRef),
    Unquote(JValRef),
    UnquoteSplice(JValRef),
    Lambda(Box<JLambda>),
    Macro(Box<JLambda>),
    Builtin(JBuiltin),
    SpecialForm(JBuiltin),
    Env(JEnvRef),
    Token(Token),
    TokenMatcher(TokenMatcher),
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
            _ => Err(JError::new(TypeError, "expected an int")),
        }
    }
    pub fn to_float(&self) -> Result<JTFloat, JError> {
        match self {
            Self::Float(x) => Ok(*x),
            _ => Err(JError::new(TypeError, "expected a float")),
        }
    }
    pub fn to_bool(&self) -> Result<bool, JError> {
        match self {
            Self::Bool(n) => Ok(*n),
            _ => Err(JError::new(TypeError, "expected a bool")),
        }
    }
    pub fn to_pair(&self) -> Result<&JPair, JError> {
        match self {
            Self::Pair(p) => Ok(p),
            _ => Err(JError::new(TypeError, "expected a pair")),
        }
    }
    pub fn to_vector(&self) -> Result<&JVector, JError> {
        match self {
            Self::Vector(v) => Ok(v),
            _ => Err(JError::new(TypeError, "expected a vector")),
        }
    }
    pub fn to_str(&self) -> Result<&str, JError> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(JError::new(TypeError, "expected a string")),
        }
    }
    pub fn to_symbol(&self) -> Result<&str, JError> {
        match self {
            Self::Symbol(s) => Ok(s),
            _ => Err(JError::new(TypeError, "expected a symbol")),
        }
    }
    pub fn to_env(&self) -> Result<JEnvRef, JError> {
        match self {
            Self::Env(e) => Ok(Rc::clone(e)),
            _ => Err(JError::new(TypeError, "expected an env")),
        }
    }
    pub fn to_error(&self) -> Result<&JError, JError> {
        match self {
            Self::Error(e) => Ok(e),
            _ => Err(JError::new(TypeError, "expected an error")),
        }
    }
    pub fn to_lambda(&self) -> Result<&JLambda, JError> {
        match self {
            Self::Lambda(l) => Ok(l.as_ref()),
            _ => Err(JError::new(TypeError, "expected a token")),
        }
    }
    pub fn to_token(&self) -> Result<&Token, JError> {
        match self {
            Self::Token(t) => Ok(t),
            _ => Err(JError::new(TypeError, "expected a token")),
        }
    }
    pub fn to_tokenmatcher(&self) -> Result<&TokenMatcher, JError> {
        match self {
            Self::TokenMatcher(t) => Ok(t),
            _ => Err(JError::new(TypeError, "expected a tokenmatcher")),
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
            _ => Err(JError::new(TypeError, "can only iter lists")),
        }
    }
}
