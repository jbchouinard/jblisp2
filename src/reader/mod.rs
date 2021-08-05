use crate::JError;
use std::fmt;

pub mod parser;
pub mod tokenizer;

#[derive(Debug)]
pub struct ReaderError {
    pos: usize,
    reason: String,
}

impl ReaderError {
    pub fn new(reason: &str, pos: usize) -> Self {
        Self {
            pos,
            reason: reason.to_string(),
        }
    }
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "Syntax error: {} at {}", self.reason, self.pos)
    }
}

impl From<ReaderError> for JError {
    fn from(re: ReaderError) -> Self {
        Self::SyntaxError {
            position: re.pos,
            reason: re.reason,
        }
    }
}

pub type Result<T> = std::result::Result<T, ReaderError>;
