use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::*;

pub type Constructor<T> = Box<dyn Fn(T) -> JValRef>;

pub struct Interned<T> {
    vals: HashMap<T, JValRef>,
    pub constructor: Constructor<T>,
}

impl<T> Interned<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(constructor: Constructor<T>) -> Self {
        Self {
            vals: HashMap::new(),
            constructor,
        }
    }
    pub fn get_or_insert(&mut self, val: T) -> JValRef {
        let cons = &self.constructor;
        Rc::clone(self.vals.entry(val.clone()).or_insert_with(|| (cons)(val)))
    }
}
