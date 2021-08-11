use std::rc::Rc;

use crate::reader::readermacro::{Matcher, ReaderMacro, TokenValueMatcher};
use crate::*;

fn transform_namespace(_v: Vec<Token>) -> Vec<Token> {
    todo!();
}

fn jreadermacro_namespace() -> ReaderMacro {
    ReaderMacro::new(
        vec![TokenValueMatcher::Ident(Matcher::Any)],
        Rc::new(transform_namespace),
    )
}

pub fn add_reader_macros(state: &mut JState) {
    state.add_reader_macro(jreadermacro_namespace());
}
