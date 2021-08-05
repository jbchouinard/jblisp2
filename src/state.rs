use std::rc::Rc;

use crate::primitives::intern::Interned;
use crate::*;

const STR_INTERN_MAX_LEN: usize = 1024;
const INT_INTERN_MAX_VAL: JTInt = 1024;

pub struct JState {
    pub const_true: JValRef,
    pub const_false: JValRef,
    interned_int: Interned<JTInt>,
    interned_sym: Interned<String>,
    interned_str: Interned<String>,
}

impl JState {
    pub fn new() -> Self {
        Self {
            const_true: JVal::Bool(true).into_ref(),
            const_false: JVal::Bool(false).into_ref(),
            interned_int: Interned::new(Box::new(JVal::_int)),
            interned_sym: Interned::new(Box::new(JVal::_sym)),
            interned_str: Interned::new(Box::new(JVal::_str)),
        }
    }
    pub fn get_bool(&self, val: bool) -> JValRef {
        if val {
            Rc::clone(&self.const_true)
        } else {
            Rc::clone(&self.const_false)
        }
    }
    pub fn make_int(&mut self, val: JTInt) -> JValRef {
        if val > INT_INTERN_MAX_VAL {
            (self.interned_int.constructor)(val)
        } else {
            self.interned_int.get_or_insert(val)
        }
    }
    pub fn make_sym(&mut self, val: String) -> JValRef {
        self.interned_sym.get_or_insert(val)
    }
    pub fn make_str(&mut self, val: String) -> JValRef {
        if val.len() > STR_INTERN_MAX_LEN {
            (self.interned_str.constructor)(val)
        } else {
            self.interned_str.get_or_insert(val)
        }
    }
}

impl Default for JState {
    fn default() -> Self {
        Self::new()
    }
}
