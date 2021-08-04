use lazy_static::lazy_static;
use regex::Regex;

use super::{ReaderError, Result};
use crate::types::JTInt;

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Whitespace,
    LParen,
    RParen,
    Quote,
    Int(JTInt),
    Ident(String),
    String(String),
    Eof,
}

pub struct Token {
    pub value: TokenValue,
    pub pos: usize,
}

impl Token {
    pub fn new(value: TokenValue, pos: usize) -> Self {
        Self { value, pos }
    }
}

lazy_static! {
    static ref RE_WS: Regex = Regex::new(r"^\s+").unwrap();
    static ref RE_LPAREN: Regex = Regex::new(r"^\(").unwrap();
    static ref RE_RPAREN: Regex = Regex::new(r"^\)").unwrap();
    static ref RE_QUOTE: Regex = Regex::new(r"^'").unwrap();
    static ref RE_INT: Regex = Regex::new(r"^-?[0-9]+").unwrap();
    static ref RE_IDENT: Regex =
        Regex::new(r"^[a-zA-Z+.*/<>=!?:$%_&~^-][0-9a-zA-Z+.*/<=>!?:$%_&~^-]*").unwrap();
    static ref RE_STRING: Regex = Regex::new(r#"^"([^"]|\\")*""#).unwrap();
}

type TResult = std::result::Result<TokenValue, String>;

fn t_lparen(_: &str) -> TResult {
    Ok(TokenValue::LParen)
}

fn t_rparen(_: &str) -> TResult {
    Ok(TokenValue::RParen)
}

fn t_quote(_: &str) -> TResult {
    Ok(TokenValue::Quote)
}

fn t_int(val: &str) -> TResult {
    match val.parse::<JTInt>() {
        Ok(n) => Ok(TokenValue::Int(n)),
        Err(e) => Err(format!("int error: {}", e)),
    }
}

fn t_ident(val: &str) -> TResult {
    Ok(TokenValue::Ident(val.to_string()))
}

fn t_string(val: &str) -> TResult {
    Ok(TokenValue::String(val[1..val.len() - 1].to_string()))
}

fn t_ws(_: &str) -> TResult {
    Ok(TokenValue::Whitespace)
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
                self.pos += mat.end();
                match cons(mat.as_str()) {
                    Ok(tokval) => Ok(Some(Token::new(tokval, spos))),
                    Err(reason) => Err(ReaderError::new(&reason, spos)),
                }
            }
            None => Ok(None),
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if self.pos >= self.input.len() {
            return Ok(Token::new(TokenValue::Eof, self.pos));
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
        if let Some(token) = self.try_token(&RE_QUOTE, t_quote)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_INT, t_int)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_IDENT, t_ident)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_STRING, t_string)? {
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
            if next.value == TokenValue::Eof {
                break;
            }
            if next.value != TokenValue::Whitespace {
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
        let tokens = tokenizer.tokenize().unwrap();
        let tokvalues: Vec<TokenValue> = tokens.into_iter().map(|t| t.value).collect();

        assert_eq!(
            vec![
                TokenValue::LParen,
                TokenValue::Ident("*".to_string()),
                TokenValue::Int(12),
                TokenValue::Int(-15),
                TokenValue::RParen,
            ],
            tokvalues
        );
    }

    #[test]
    fn test_tokenizer_2() {
        let input = "(concat \"foo\" \"bar\")";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        let tokvalues: Vec<TokenValue> = tokens.into_iter().map(|t| t.value).collect();

        assert_eq!(
            vec![
                TokenValue::LParen,
                TokenValue::Ident("concat".to_string()),
                TokenValue::String("foo".to_string()),
                TokenValue::String("bar".to_string()),
                TokenValue::RParen,
            ],
            tokvalues
        );
    }

    #[test]
    fn test_tokenizer_3() {
        let input = "(quote '(1 2 3))";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        let tokvalues: Vec<TokenValue> = tokens.into_iter().map(|t| t.value).collect();

        assert_eq!(
            vec![
                TokenValue::LParen,
                TokenValue::Ident("quote".to_string()),
                TokenValue::Quote,
                TokenValue::LParen,
                TokenValue::Int(1),
                TokenValue::Int(2),
                TokenValue::Int(3),
                TokenValue::RParen,
                TokenValue::RParen,
            ],
            tokvalues
        );
    }
}
