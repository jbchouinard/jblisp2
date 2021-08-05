use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct JEnv {
    parent: Option<JEnvRef>,
    vars: RefCell<HashMap<String, JValRef>>,
}

pub type JEnvRef = Rc<JEnv>;

impl JEnv {
    pub fn new(parent: Option<JEnvRef>) -> Self {
        Self {
            parent,
            vars: RefCell::new(HashMap::new()),
        }
    }

    /// Look for value of binding.
    pub fn lookup(&self, v: &str) -> Option<JValRef> {
        match self.vars.borrow().get(v) {
            Some(val) => Some(Rc::clone(val)),
            None => match &self.parent {
                Some(parent) => parent.lookup(v),
                None => None,
            },
        }
    }

    pub fn try_lookup(&self, v: &str) -> JResult {
        self.lookup(v)
            .ok_or_else(|| JError::UndefError(v.to_string()))
    }

    /// Create a new binding.
    pub fn define(&self, v: &str, val: JValRef) {
        self.vars.borrow_mut().insert(v.to_string(), val);
    }

    /// Change existing binding.
    pub fn set(&self, v: &str, val: JValRef) -> Result<(), JError> {
        if self.vars.borrow().contains_key(v) {
            self.vars.borrow_mut().insert(v.to_string(), val);
            Ok(())
        } else {
            match &self.parent {
                Some(penv) => penv.set(v, val),
                None => Err(JError::UndefError(v.to_string())),
            }
        }
    }

    pub fn into_ref(self) -> JEnvRef {
        Rc::new(self)
    }
}

impl Default for env::JEnv {
    fn default() -> Self {
        Self::new(None)
    }
}
