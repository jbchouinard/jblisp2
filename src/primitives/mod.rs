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

pub type JBuiltinFn = Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>;

#[derive(Clone)]
pub struct JBuiltin {
    pub name: String,
    pub f: JBuiltinFn,
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

#[allow(clippy::ptr_eq)]
impl PartialEq for JBuiltin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && &*self.f as *const dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult as *const u8
                == &*other.f as *const dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult as *const u8
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
                return Err(JError::EvalError("ill-formed params".to_string()));
            }
        }
        Ok(match rest {
            Some(rname) => Self::Variadic(names, rname),
            None => Self::Fixed(names),
        })
    }
    fn nargs(&self) -> String {
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
                JError::ApplyError(format!("expected {} argument(s)", self.nargs()))
            })?;
            env.define(p, pair.car());
            head = pair.cdr();
        }
        match self {
            Self::Variadic(_, p) => env.define(p, head),
            Self::Fixed(_) => match &*head {
                JVal::Nil => (),
                _ => {
                    return Err(JError::ApplyError(format!(
                        "expected {} argument(s)",
                        self.nargs()
                    )))
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
    pub fn to_pair(&self) -> Result<&JPair, JError> {
        match self {
            Self::Pair(p) => Ok(p),
            _ => Err(JError::TypeError("expected a pair".to_string())),
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
