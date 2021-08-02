use std::collections::HashMap;

use crate::builtin::add_builtins;
use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct JEnv {
    parent: Option<Box<JEnv>>,
    vars: HashMap<String, JValue>,
}

impl JEnv {
    pub fn new() -> Self {
        Self {
            parent: None,
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, v: &str) -> Option<JValue> {
        match self.vars.get(v) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(v),
                None => None,
            },
        }
    }

    pub fn set(&mut self, v: &str, val: JValue) {
        self.vars.insert(v.to_string(), val);
    }

    pub fn set_parent(&mut self, parent: Option<Box<JEnv>>) {
        self.parent = parent
    }
}

impl Default for env::JEnv {
    fn default() -> Self {
        let mut env = Self::new();
        add_builtins(&mut env);
        env
    }
}
