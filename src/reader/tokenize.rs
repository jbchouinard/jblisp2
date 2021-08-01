use lazy_static::lazy_static;
use regex::Regex;

use super::{ReaderError, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Int(i64),
}

lazy_static! {
    static ref RE_WS: Regex = Regex::new(r"^\s+").unwrap();
    static ref RE_LPAREN: Regex = Regex::new(r"^\(").unwrap();
    static ref RE_RPAREN: Regex = Regex::new(r"^\)").unwrap();
    static ref RE_INT: Regex = Regex::new(r"^[0-9]+").unwrap();
}

fn t_lparen() -> Token {
    Token::LParen
}

fn t_rparen() -> Token {
    Token::RParen
}

fn t_int(val: &str) -> Result<Token> {
    match val.parse::<i64>() {
        Ok(n) => Ok(Token::Int(n)),
        _ => Err(ReaderError::new("invalid int")),
    }
}

fn next_token(line: String) -> Result<(Option<Token>, String)> {
    if let Some(mat) = RE_WS.find(&line) {
        Ok((None, line[mat.end()..].to_string()))
    } else if RE_LPAREN.is_match(&line) {
        Ok((Some(t_lparen()), line[1..].to_string()))
    } else if RE_RPAREN.is_match(&line) {
        Ok((Some(t_rparen()), line[1..].to_string()))
    } else if let Some(mat) = RE_INT.find(&line) {
        Ok((Some(t_int(mat.as_str())?), line[mat.end()..].to_string()))
    } else {
        Err(ReaderError::new("unexpected character"))
    }
}

pub fn tokenize(mut line: String) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    while !line.is_empty() {
        let (tok, rest) = next_token(line)?;
        if let Some(t) = tok {
            tokens.push(t);
        }
        line = rest;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let actual = tokenize("( 12 45 )".to_string()).unwrap();
        let expected = vec![Token::LParen, Token::Int(12), Token::Int(45), Token::RParen];
        assert_eq!(expected, actual);
    }
}
