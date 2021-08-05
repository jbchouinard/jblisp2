use crate::JError;
use std::fmt;

pub mod parser;
pub mod tokenizer;

#[derive(Debug, PartialEq, Clone)]
pub struct PositionTag {
    filename: String,
    lineno: usize,
    col: usize,
}

impl fmt::Display for PositionTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}:{}", self.filename, self.lineno, self.col)
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub position: PositionTag,
    pub reason: String,
}

impl ParserError {
    pub fn new(filename: &str, lineno: usize, col: usize, reason: &str) -> Self {
        Self {
            position: PositionTag {
                filename: filename.to_string(),
                lineno,
                col,
            },
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "Error parsing {} {}", self.position, self.reason)
    }
}

impl From<ParserError> for JError {
    fn from(pe: ParserError) -> Self {
        Self::SyntaxError {
            position: pe.position.clone(),
            reason: pe.reason,
        }
    }
}
