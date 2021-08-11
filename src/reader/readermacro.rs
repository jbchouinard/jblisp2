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

pub type TokenTransformer = Rc<dyn Fn(Vec<Token>) -> Vec<Token>>;

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
}

pub struct ReaderMacroIterator {
    tokens: Box<dyn TokenIter>,
    rm: ReaderMacro,
    buffer_in: VecDeque<Token>,
    buffer_out: VecDeque<Token>,
}

impl ReaderMacroIterator {
    pub fn new(tokens: Box<dyn TokenIter>, rm: ReaderMacro) -> Self {
        Self {
            tokens,
            rm,
            buffer_in: VecDeque::new(),
            buffer_out: VecDeque::new(),
        }
    }
}

impl TokenIter for ReaderMacroIterator {
    fn next_token(&mut self) -> Result<Token, TokenError> {
        // TODO
        self.tokens.next_token()
    }
}
