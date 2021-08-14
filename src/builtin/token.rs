use crate::reader::readermacro::{Matcher, TokenMatcher};
use crate::*;

fn str_to_char(s: &str) -> Result<char, JError> {
    if s.len() == 1 {
        Ok(s.chars().next().unwrap())
    } else {
        Err(JError::new(TypeError, "expected a single char"))
    }
}

pub fn jspecial_token(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut args: Vec<JValRef> = args.iter_list()?.collect();
    let (v1, v2) = if args.len() == 1 {
        (args.pop().unwrap(), state.nil())
    } else if args.len() == 2 {
        let second = args.pop().unwrap();
        let first = args.pop().unwrap();
        (first, second)
    } else {
        return Err(JError::new(ApplyError, "expected 1 or 2 arguments"));
    };
    let v1 = v1.to_symbol()?;
    match (v1, &*v2) {
        ("lparen", JVal::Nil) => state.token(TokenValue::LParen),
        ("rparen", JVal::Nil) => state.token(TokenValue::RParen),
        ("quote", JVal::Nil) => state.token(TokenValue::Quote),
        ("eof", JVal::Nil) => state.token(TokenValue::Eof),
        ("string", JVal::String(s)) => state.token(TokenValue::String(s.clone())),
        ("ident", JVal::Symbol(s)) => state.token(TokenValue::Ident(s.clone())),
        ("int", JVal::Int(n)) => state.token(TokenValue::Int(*n)),
        ("float", JVal::Float(x)) => state.token(TokenValue::Float(*x)),
        ("char", JVal::String(c)) => state.token(TokenValue::Anychar(str_to_char(c)?)),
        _ => Err(JError::new(TypeError, "invalid token definition")),
    }
}

pub fn jspecial_tokenmatcher(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut args: Vec<JValRef> = args.iter_list()?.collect();
    let (v1, v2) = if args.len() == 1 {
        (args.pop().unwrap(), state.nil())
    } else if args.len() == 2 {
        let second = args.pop().unwrap();
        let first = args.pop().unwrap();
        (first, second)
    } else {
        return Err(JError::new(ApplyError, "expected 1 or 2 arguments"));
    };
    let v1 = v1.to_symbol()?;
    Ok(JVal::TokenMatcher(match (v1, &*v2) {
        ("any", JVal::Nil) => TokenMatcher::Any,
        ("lparen", JVal::Nil) => TokenMatcher::LParen,
        ("rparen", JVal::Nil) => TokenMatcher::RParen,
        ("quote", JVal::Nil) => TokenMatcher::Quote,
        ("eof", JVal::Nil) => TokenMatcher::Eof,
        ("string", JVal::String(s)) => TokenMatcher::String(Matcher::Exact(s.clone())),
        ("string", JVal::Nil) => TokenMatcher::String(Matcher::Any),
        ("ident", JVal::Symbol(s)) => TokenMatcher::Ident(Matcher::Exact(s.clone())),
        ("ident", JVal::Nil) => TokenMatcher::Ident(Matcher::Any),
        ("int", JVal::Int(n)) => TokenMatcher::Int(Matcher::Exact(*n)),
        ("int", JVal::Nil) => TokenMatcher::Int(Matcher::Any),
        ("float", JVal::Float(x)) => TokenMatcher::Float(Matcher::Exact(*x)),
        ("float", JVal::Nil) => TokenMatcher::Float(Matcher::Any),
        ("char", JVal::String(c)) => TokenMatcher::Anychar(str_to_char(c)?),
        _ => return Err(JError::new(TypeError, "invalid token matcher definition")),
    })
    .into_ref())
}
