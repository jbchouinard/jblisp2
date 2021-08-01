use std::fmt;

pub mod tokenize;

#[derive(Debug)]
pub struct ReaderError {
    reason: String,
}

impl ReaderError {
    pub fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "Reader error: {}", self.reason)
    }
}

pub type Result<T> = std::result::Result<T, ReaderError>;
