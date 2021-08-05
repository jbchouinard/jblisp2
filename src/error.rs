use std::fmt;

use crate::reader::PositionTag;
use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum JError {
    Exception(String),
    AssertionError(String),
    TypeError(String),
    EvalError(String),
    ApplyError(String),
    UndefError(String),
    OsError(String),
    SyntaxError {
        position: PositionTag,
        reason: String,
    },
}

impl fmt::Display for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Exception(s) => write!(f, "Exception \"{}\"", s),
            Self::AssertionError(s) => write!(f, "AssertionError \"{}\"", s),
            Self::TypeError(s) => write!(f, "TypeError \"{}\"", s),
            Self::EvalError(s) => write!(f, "EvalError \"{}\"", s),
            Self::ApplyError(s) => write!(f, "ApplyError \"{}\"", s),
            Self::UndefError(s) => write!(f, "UndefError {}", s),
            Self::OsError(s) => write!(f, "OsError \"{}\"", s),
            Self::SyntaxError { position, reason } => {
                write!(f, "SyntaxError \"{} at {}\"", reason, position)
            }
        }
    }
}

pub type JResult = Result<JValRef, JError>;
