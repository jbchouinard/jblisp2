use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static ENV_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn env_id() -> usize {
    ENV_COUNTER.fetch_add(1, Ordering::SeqCst)
}

use crate::*;

#[derive(Clone)]
pub struct JEnv {
    id: usize,
    pub parent: Option<JEnvRef>,
    vars: RefCell<HashMap<String, JValRef>>,
}

pub type JEnvRef = Rc<JEnv>;

impl JEnv {
    pub fn new(parent: Option<JEnvRef>) -> Self {
        Self {
            id: env_id(),
            parent,
            vars: RefCell::new(HashMap::new()),
        }
    }

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
        self.lookup(v).ok_or_else(|| JError::new(NotDefined, v))
    }

    /// Create a new binding.
    pub fn define(&self, v: &str, val: JValRef) {
        self.vars.borrow_mut().insert(v.to_string(), val);
    }

    /// Change existing binding.
    pub fn set(&self, v: &str, val: JValRef, state: &mut JState) -> Result<(), JError> {
        if self.vars.borrow().contains_key(v) {
            self.vars.borrow_mut().insert(v.to_string(), val);
            Ok(())
        } else {
            match &self.parent {
                Some(penv) => penv.set(v, val, state),
                None => Err(JError::new(NotDefined, v)),
            }
        }
    }

    pub fn into_ref(self) -> JEnvRef {
        Rc::new(self)
    }
}

impl PartialEq for JEnv {
    fn eq(&self, other: &JEnv) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for JEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#[env<{}>]", self.id)
    }
}

impl fmt::Debug for JEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut parts = vec!["{".to_string()];
        for (k, v) in self.vars.borrow().iter() {
            parts.push(format!("    {}: {}", k, v))
        }
        parts.push("}".to_string());
        write!(f, "{}", parts.join("\n"))
    }
}

impl Default for env::JEnv {
    fn default() -> Self {
        Self::new(None)
    }
}
