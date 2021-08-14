use std::convert::TryInto;
use std::rc::Rc;

use crate::reader::readermacro::{Matcher, ReaderMacro, TokenMatcher};
use crate::*;

pub fn get_n_tokens<const N: usize>(args: Vec<Token>) -> [Token; N] {
    args.try_into().unwrap()
}

fn transform_vector(tokens: Vec<Token>) -> Result<Vec<Token>, JError> {
    let [hash, paren] = get_n_tokens(tokens);
    Ok(vec![
        Token::new(TokenValue::LParen, hash.pos),
        Token::new(TokenValue::Ident("vector".to_string()), paren.pos),
    ])
}

fn jreadermacro_vector() -> ReaderMacro {
    ReaderMacro::new(
        vec![TokenMatcher::Anychar('#'), TokenMatcher::LParen],
        Rc::new(transform_vector),
    )
}

fn transform_namespace(tokens: Vec<Token>) -> Result<Vec<Token>, JError> {
    let [ident] = get_n_tokens(tokens);
    let sym = match ident.value {
        TokenValue::Ident(ref s) => s.to_string(),
        _ => panic!(),
    };
    let subsyms: Vec<&str> = sym.split("::").collect();
    if subsyms.len() == 1 {
        Ok(vec![ident])
    } else {
        let mut expr = vec![
            Token::new(TokenValue::LParen, ident.pos.clone()),
            Token::new(
                TokenValue::Ident("env-lookup-rec".to_string()),
                ident.pos.clone(),
            ),
        ];
        for sym in subsyms {
            expr.push(Token::new(
                TokenValue::Ident(sym.to_string()),
                ident.pos.clone(),
            ))
        }
        expr.push(Token::new(TokenValue::RParen, ident.pos));
        Ok(expr)
    }
}

fn jreadermacro_namespace() -> ReaderMacro {
    ReaderMacro::new(
        vec![TokenMatcher::Ident(Matcher::Any)],
        Rc::new(transform_namespace),
    )
}

pub fn add_reader_macros(state: &mut JState) {
    state.add_reader_macro(jreadermacro_vector());
    state.add_reader_macro(jreadermacro_namespace());
}
