use std::fmt;
use std::rc::Rc;

use crate::*;

#[derive(Clone, Debug)]
pub enum JCallable {
    Lambda(String, JLambda),
    Builtin(String, JBuiltin),
}

impl JCallable {
    pub fn from_jval(val: JValRef) -> Option<Self> {
        match &*val {
            JVal::Lambda(l) => Some(Self::Lambda("lambda".to_string(), l.as_ref().clone())),
            JVal::Macro(l) => Some(Self::Lambda("macro".to_string(), l.as_ref().clone())),
            JVal::Builtin(b) => Some(Self::Builtin("builtin".to_string(), b.clone())),
            JVal::SpecialForm(b) => Some(Self::Builtin("specialform".to_string(), b.clone())),
            _ => None,
        }
    }
}

impl fmt::Display for JCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lambda(t, l) => match &l.name {
                Some(name) => write!(f, "{} {}", t, name),
                None => write!(f, "unnamed {}", t),
            },
            Self::Builtin(t, b) => write!(f, "{} {}", t, b.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TracebackFrame {
    pos: Option<PositionTag>,
    env: JEnvRef,
    proc: JCallable,
}

impl TracebackFrame {
    pub fn from_jval(val: JValRef, env: JEnvRef) -> Option<Self> {
        use JCallable::*;
        let proc = JCallable::from_jval(val);
        match proc {
            Some(Lambda(_, ref l)) => Some(Self {
                pos: l.defpos.clone(),
                env: Rc::clone(&l.closure),
                proc: proc.unwrap(),
            }),
            Some(Builtin(_, _)) => Some(Self {
                pos: None,
                env: Rc::clone(&env),
                proc: proc.unwrap(),
            }),
            None => None,
        }
    }
}

impl fmt::Display for TracebackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.pos {
            Some(pos) => write!(
                f,
                "File \"{}\", line {}, in {}",
                pos.filename, pos.lineno, self.proc
            ),
            None => write!(f, "In {}", self.proc),
        }
    }
}
