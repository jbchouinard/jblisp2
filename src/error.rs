use std::fmt;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum JErrorKind {
    Exception,
    AssertionError,
    TypeError,
    EvalError,
    ApplyError,
    NotDefined,
    OsError,
    SyntaxError,
    UserDefined(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct JError {
    pub kind: JErrorKind,
    pub reason: String,
}

impl fmt::Display for JErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            JErrorKind::Exception => write!(f, "Exception"),
            JErrorKind::AssertionError => write!(f, "AssertionError"),
            JErrorKind::TypeError => write!(f, "TypeError"),
            JErrorKind::EvalError => write!(f, "EvalError"),
            JErrorKind::ApplyError => write!(f, "ApplyError"),
            JErrorKind::NotDefined => write!(f, "NotDefined"),
            JErrorKind::OsError => write!(f, "OsError"),
            JErrorKind::SyntaxError => write!(f, "SyntaxError"),
            JErrorKind::UserDefined(s) => write!(f, "{}", s),
        }
    }
}

impl JError {
    pub fn new(kind: JErrorKind, reason: &str) -> Self {
        JError {
            kind,
            reason: reason.to_string(),
        }
    }
    pub fn is_same_kind(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Display for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.kind, self.reason)
    }
}

pub type JResult = Result<JValRef, JError>;
