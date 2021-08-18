use std::rc::Rc;

use crate::builtin::get_n_args;
use crate::reader::readermacro::{LambdaTokenTransformer, Matcher, ReaderMacro, TokenMatcher};
use crate::*;

fn str_to_char(s: &str) -> Result<char, JError> {
    if s.len() == 1 {
        Ok(s.chars().next().unwrap())
    } else {
        Err(JError::new(TypeError, "expected a single char"))
    }
}

pub fn jbuiltin_token(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
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
        ("char", JVal::String(c)) => state.token(TokenValue::Char(str_to_char(c)?)),
        ("string", JVal::String(s)) => state.token(TokenValue::String(s.clone())),
        ("ident", JVal::Symbol(s)) => state.token(TokenValue::Ident(s.clone())),
        ("ident", JVal::String(s)) => state.token(TokenValue::Ident(s.clone())),
        ("int", JVal::Int(n)) => state.token(TokenValue::Int(*n)),
        ("float", JVal::Float(x)) => state.token(TokenValue::Float(*x)),
        ("eof", JVal::Nil) => state.token(TokenValue::Eof),
        _ => Err(JError::new(TypeError, "invalid token definition")),
    }
}

pub fn jbuiltin_tokenmatcher(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
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
        ("eof", JVal::Nil) => TokenMatcher::Eof,
        ("string", JVal::String(s)) => TokenMatcher::String(Matcher::Exact(s.clone())),
        ("string", JVal::Nil) => TokenMatcher::String(Matcher::Any),
        ("ident", JVal::Symbol(s)) => TokenMatcher::Ident(Matcher::Exact(s.clone())),
        ("ident", JVal::String(s)) => TokenMatcher::Ident(Matcher::Exact(s.clone())),
        ("ident", JVal::Nil) => TokenMatcher::Ident(Matcher::Any),
        ("int", JVal::Int(n)) => TokenMatcher::Int(Matcher::Exact(*n)),
        ("int", JVal::Nil) => TokenMatcher::Int(Matcher::Any),
        ("float", JVal::Float(x)) => TokenMatcher::Float(Matcher::Exact(*x)),
        ("float", JVal::Nil) => TokenMatcher::Float(Matcher::Any),
        ("char", JVal::String(c)) => TokenMatcher::Char(str_to_char(c)?),
        _ => return Err(JError::new(TypeError, "invalid token matcher definition")),
    })
    .into_ref())
}

pub fn jbuiltin_token_type(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [tok] = get_n_args(args)?;
    let tok = tok.to_token()?;
    Ok(state.symbol(
        match &tok.value {
            TokenValue::Eof => "eof",
            TokenValue::String(_) => "string",
            TokenValue::Ident(_) => "ident",
            TokenValue::Int(_) => "int",
            TokenValue::Float(_) => "float",
            TokenValue::Char(_) => "char",
        }
        .to_string(),
    ))
}

pub fn jbuiltin_token_value(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [tok] = get_n_args(args)?;
    let tok = tok.to_token()?;
    Ok(match &tok.value {
        TokenValue::String(s) => state.string(s.clone()),
        TokenValue::Ident(s) => state.symbol(s.clone()),
        TokenValue::Int(n) => state.int(*n),
        TokenValue::Float(x) => state.float(*x),
        TokenValue::Char(c) => state.string(c.to_string()),
        _ => state.nil(),
    })
}

pub fn jbuiltin_install_reader_macro(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let mut matchers: Vec<JValRef> = args.iter_list()?.collect();
    if matchers.len() < 2 {
        return Err(JError::new(ApplyError, "expected at least 2 arguments"));
    }
    let transformer = matchers.pop().unwrap().to_lambda()?.clone();
    let matchers = matchers
        .iter()
        .map(|v| v.to_tokenmatcher().map(|t| t.clone()))
        .collect::<Result<Vec<TokenMatcher>, JError>>()?;

    state.add_reader_macro(ReaderMacro::new(
        matchers,
        Rc::new(LambdaTokenTransformer::new(transformer, env)),
    ));
    Ok(state.nil())
}
