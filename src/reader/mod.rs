use std::fmt;

use crate::{JError, SyntaxError};

pub mod parser;
pub mod readermacro;
pub mod tokenizer;

#[derive(Debug, PartialEq, Clone)]
pub struct PositionTag {
    pub filename: String,
    pub lineno: usize,
    pub col: usize,
}

impl PositionTag {
    pub fn new(filename: &str, lineno: usize, col: usize) -> Self {
        Self {
            filename: filename.to_string(),
            lineno,
            col,
        }
    }
}

impl fmt::Display for PositionTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}:{}", self.filename, self.lineno, self.col)
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub pos: PositionTag,
    pub reason: String,
}

impl ParserError {
    pub fn new(pt: PositionTag, reason: &str) -> Self {
        Self {
            pos: pt,
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "Error parsing {} {}", self.pos, self.reason)
    }
}

impl From<ParserError> for JError {
    fn from(pe: ParserError) -> Self {
        Self {
            kind: SyntaxError,
            reason: format!("{} at {}", pe.reason, pe.pos),
        }
    }
}
