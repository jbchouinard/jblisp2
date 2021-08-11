use std::fmt;
use std::rc::Rc;

use crate::*;

#[derive(Clone, Debug)]
pub struct TracebackFrame {
    pos: Option<PositionTag>,
    env: JEnvRef,
    proc: JValRef,
}

impl TracebackFrame {
    pub fn from_lambda(val: JValRef) -> Option<Self> {
        let (pos, env, proc) = match &*val {
            JVal::Lambda(l) => (l.defpos.clone(), Rc::clone(&l.closure), Rc::clone(&val)),
            JVal::ProcMacro(l) => (l.defpos.clone(), Rc::clone(&l.closure), Rc::clone(&val)),
            _ => return None,
        };
        Some(Self { pos, env, proc })
    }
    pub fn from_builtin(val: JValRef, env: JEnvRef) -> Option<Self> {
        let proc = match &*val {
            JVal::Builtin(_) => val,
            JVal::SpecialForm(_) => val,
            _ => return None,
        };
        Some(Self {
            pos: None,
            proc,
            env,
        })
    }
    pub fn from_any(val: JValRef, env: JEnvRef) -> Option<Self> {
        match &*val {
            JVal::Lambda(_) => Self::from_lambda(val),
            JVal::ProcMacro(_) => Self::from_lambda(val),
            JVal::Builtin(_) => Self::from_builtin(val, env),
            JVal::SpecialForm(_) => Self::from_builtin(val, env),
            _ => None,
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
