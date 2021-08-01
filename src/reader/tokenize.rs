use lazy_static::lazy_static;
use regex::Regex;

use super::{ReaderError, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    Whitespace,
    LParen,
    RParen,
    Int(i64),
    Ident(String),
    Eof,
}

lazy_static! {
    static ref RE_WS: Regex = Regex::new(r"^\s+").unwrap();
    static ref RE_LPAREN: Regex = Regex::new(r"^\(").unwrap();
    static ref RE_RPAREN: Regex = Regex::new(r"^\)").unwrap();
    static ref RE_INT: Regex = Regex::new(r"^-?[0-9]+").unwrap();
    static ref RE_IDENT: Regex = Regex::new(r"^[a-z%=~<>?!*/+-][0-9a-z%=~<>?!*/+-]*").unwrap();
}

type TResult = std::result::Result<Token, String>;

fn t_lparen(_: &str) -> TResult {
    Ok(Token::LParen)
}

fn t_rparen(_: &str) -> TResult {
    Ok(Token::RParen)
}

fn t_int(val: &str) -> TResult {
    match val.parse::<i64>() {
        Ok(n) => Ok(Token::Int(n)),
        Err(e) => Err(format!("int error: {}", e)),
    }
}

fn t_ident(val: &str) -> TResult {
    Ok(Token::Ident(val.to_string()))
}

fn t_ws(_: &str) -> TResult {
    Ok(Token::Whitespace)
}

pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn try_token<T>(&mut self, re: &Regex, cons: T) -> Result<Option<Token>>
    where
        T: Fn(&str) -> TResult,
    {
        match re.find(&self.input[self.pos..]) {
            Some(mat) => {
                let spos = self.pos;
                self.pos = self.pos + mat.end();
                match cons(mat.as_str()) {
                    Ok(token) => Ok(Some(token)),
                    Err(reason) => Err(ReaderError::new(&reason, spos)),
                }
            }
            None => Ok(None),
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if self.pos >= self.input.len() {
            return Ok(Token::Eof);
        }
        if let Some(token) = self.try_token(&RE_WS, t_ws)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_LPAREN, t_lparen)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_RPAREN, t_rparen)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_INT, t_int)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_IDENT, t_ident)? {
            return Ok(token);
        }
        Err(ReaderError::new(
            &format!(
                "unexpected character {}",
                &self.input[self.pos..self.pos + 1]
            ),
            self.pos,
        ))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];
        loop {
            let next = self.next_token()?;
            if next == Token::Eof {
                break;
            }
            if next != Token::Whitespace {
                tokens.push(next);
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_1() {
        let input = "(* 12 -15)";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            vec![
                Token::LParen,
                Token::Ident("*".to_string()),
                Token::Int(12),
                Token::Int(-15),
                Token::RParen,
            ],
            tokenizer.tokenize().unwrap()
        );
    }
}
