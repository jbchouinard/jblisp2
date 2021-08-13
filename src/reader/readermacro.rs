use std::collections::VecDeque;
use std::rc::Rc;

use crate::*;

#[derive(Debug, Clone)]
pub enum Matcher<T> {
    Any,
    Exact(T),
}

impl<T: PartialEq> Matcher<T> {
    pub fn matches(&self, val: &T) -> bool {
        match self {
            Matcher::Any => true,
            Matcher::Exact(m) => m == val,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenValueMatcher {
    Any,
    LParen,
    RParen,
    Quote,
    Int(Matcher<JTInt>),
    Ident(Matcher<String>),
    String(Matcher<String>),
    Eof,
    Anychar(char),
}

impl TokenValueMatcher {
    pub fn matches(&self, tv: &TokenValue) -> bool {
        use TokenValueMatcher::*;
        match (self, tv) {
            (Any, _) => true,
            (LParen, TokenValue::LParen) => true,
            (RParen, TokenValue::RParen) => true,
            (Quote, TokenValue::Quote) => true,
            (Eof, TokenValue::Eof) => true,
            (Anychar(c1), TokenValue::Anychar(c2)) => c1 == c2,
            (Int(m), TokenValue::Int(n)) => m.matches(n),
            (Ident(m), TokenValue::Ident(s)) => m.matches(s),
            (String(m), TokenValue::String(s)) => m.matches(s),
            _ => false,
        }
    }
}

pub type TokenTransformer = Rc<dyn Fn(Vec<Token>) -> Result<Vec<Token>, JError>>;

#[derive(Clone)]
pub struct ReaderMacro {
    rule: Vec<TokenValueMatcher>,
    transformer: TokenTransformer,
}

impl ReaderMacro {
    pub fn new(rule: Vec<TokenValueMatcher>, transformer: TokenTransformer) -> Self {
        Self { rule, transformer }
    }
    pub fn rule_len(&self) -> usize {
        self.rule.len()
    }
    pub fn wrap(&self, tokens: Box<dyn TokenIter>) -> ReaderMacroIterator {
        ReaderMacroIterator::new(tokens, self.clone())
    }
    pub fn matches_rule(&self, tokens: &[Token]) -> bool {
        if tokens.len() != self.rule.len() {
            return false;
        }
        for (tok, matcher) in tokens.iter().zip(self.rule.iter()) {
            if !matcher.matches(&tok.value) {
                return false;
            }
        }
        true
    }
    pub fn apply_rule(&self, tokens: Vec<Token>) -> Result<Vec<Token>, JError> {
        (self.transformer)(tokens)
    }
}

pub struct ReaderMacroIterator {
    tokens: Box<dyn TokenIter>,
    rm: ReaderMacro,
    buffer_in: VecDeque<Token>,
    buffer_out: VecDeque<Token>,
}

impl ReaderMacroIterator {
    pub fn new(tokens: Box<dyn TokenIter>, rm: ReaderMacro) -> Self {
        let rl = rm.rule.len();
        Self {
            tokens,
            rm,
            buffer_in: VecDeque::with_capacity(rl),
            buffer_out: VecDeque::new(),
        }
    }
}

impl TokenIter for ReaderMacroIterator {
    fn next_token(&mut self) -> Result<Token, TokenError> {
        loop {
            if !self.buffer_out.is_empty() {
                return Ok(self.buffer_out.pop_front().unwrap());
            }
            while self.buffer_in.len() < self.rm.rule.len() {
                self.buffer_in.push_back(self.tokens.next_token()?)
            }
            let buf = self.buffer_in.make_contiguous();
            if self.rm.matches_rule(buf) {
                let toks_in: Vec<Token> = std::mem::replace(
                    &mut self.buffer_in,
                    VecDeque::with_capacity(self.rm.rule.len()),
                )
                .into();
                let spos = toks_in[0].pos.clone();
                self.buffer_out = match self.rm.apply_rule(toks_in) {
                    Ok(toks_out) => toks_out.into(),
                    Err(je) => return Err(TokenError::new(&format!("{}", je), spos)),
                };
            } else {
                return Ok(self.buffer_in.pop_front().unwrap());
            }
        }
    }
}
