use std::fmt;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct JError {
    pub etype: String,
    pub emsg: String,
}

impl JError {
    pub fn new(etype: &str, emsg: &str) -> Self {
        Self {
            etype: etype.to_string(),
            emsg: emsg.to_string(),
        }
    }
}

impl fmt::Display for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.etype, self.emsg)
    }
}

pub type JResult = Result<JValRef, JError>;
