use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct JEnv {
    parent: Option<JEnvRef>,
    vars: RefCell<HashMap<String, JValue>>,
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
    pub fn lookup(&self, v: &str) -> Option<JValue> {
        match self.vars.borrow().get(v) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.lookup(v),
                None => None,
            },
        }
    }

    /// Create a new binding.
    pub fn define(&self, v: &str, val: JValue) {
        self.vars.borrow_mut().insert(v.to_string(), val);
    }

    /// Change existing binding.
    pub fn set(&self, _v: &str, _val: JValue) {
        todo!()
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
