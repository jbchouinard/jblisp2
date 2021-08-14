use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

use crate::*;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenMatcher {
    Any,
    LParen,
    RParen,
    Quote,
    Int(Matcher<JTInt>),
    Float(Matcher<JTFloat>),
    Ident(Matcher<String>),
    String(Matcher<String>),
    Eof,
    Anychar(char),
}

impl TokenMatcher {
    pub fn matches(&self, tv: &TokenValue) -> bool {
        use TokenMatcher::*;
        match (self, tv) {
            (Any, _) => true,
            (LParen, TokenValue::LParen) => true,
            (RParen, TokenValue::RParen) => true,
            (Quote, TokenValue::Quote) => true,
            (Eof, TokenValue::Eof) => true,
            (Anychar(c1), TokenValue::Anychar(c2)) => c1 == c2,
            (Int(m), TokenValue::Int(n)) => m.matches(n),
            (Float(m), TokenValue::Float(n)) => m.matches(n),
            (Ident(m), TokenValue::Ident(s)) => m.matches(s),
            (String(m), TokenValue::String(s)) => m.matches(s),
            _ => false,
        }
    }
}

impl fmt::Display for TokenMatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenMatcher::*;
        match &self {
            Any => write!(f, "#ANY"),
            LParen => write!(f, "LPAREN"),
            RParen => write!(f, "RPAREN"),
            Quote => write!(f, "QUOTE"),
            Int(Matcher::Any) => write!(f, "INT(#ANY)"),
            Int(Matcher::Exact(n)) => write!(f, "INT({})", n),
            Float(Matcher::Any) => write!(f, "FLOAT(#ANY)"),
            Float(Matcher::Exact(x)) => write!(f, "FLOAT({})", x),
            Ident(Matcher::Exact(s)) => write!(f, "IDENT({})", s),
            Ident(Matcher::Any) => write!(f, "IDENT(#ANY)"),
            String(Matcher::Exact(s)) => write!(f, "STRING(\"{}\")", s),
            String(Matcher::Any) => write!(f, "String(#ANY)"),
            Eof => write!(f, "EOF"),
            Anychar(c) => write!(f, "CHAR('{}')", c),
        }
    }
}

pub trait TokenTransformer {
    fn transform(&self, tokens: Vec<Token>, state: &mut JState) -> Result<Vec<Token>, JError>;
}

impl<T: Fn(Vec<Token>) -> Result<Vec<Token>, JError>> TokenTransformer for T {
    fn transform(&self, tokens: Vec<Token>, _: &mut JState) -> Result<Vec<Token>, JError> {
        (self)(tokens)
    }
}

// pub struct LambdaTokenTransformer {
//     lambda: JLambda,
//     env: JEnvRef,
// }

#[derive(Clone)]
pub struct ReaderMacro {
    rule: Vec<TokenMatcher>,
    transformer: Rc<dyn TokenTransformer>,
}

impl ReaderMacro {
    pub fn new(rule: Vec<TokenMatcher>, transformer: Rc<dyn TokenTransformer>) -> Self {
        Self { rule, transformer }
    }
    pub fn rule_len(&self) -> usize {
        self.rule.len()
    }
    pub fn apply(&self, tokeniter: Box<dyn TokenProducer>) -> ReaderMacroIterator {
        ReaderMacroIterator::new(tokeniter, self.clone())
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
    pub fn apply_rule(&self, tokens: Vec<Token>, state: &mut JState) -> Result<Vec<Token>, JError> {
        self.transformer.transform(tokens, state)
    }
}

pub struct ReaderMacroIterator {
    tokens: Box<dyn TokenProducer>,
    rm: ReaderMacro,
    buffer_in: VecDeque<Token>,
    buffer_out: VecDeque<Token>,
}

impl ReaderMacroIterator {
    pub fn new(tokens: Box<dyn TokenProducer>, rm: ReaderMacro) -> Self {
        let rl = rm.rule.len();
        Self {
            tokens,
            rm,
            buffer_in: VecDeque::with_capacity(rl),
            buffer_out: VecDeque::new(),
        }
    }
}

impl TokenProducer for ReaderMacroIterator {
    fn next_token(&mut self, state: &mut JState) -> Result<Token, TokenError> {
        loop {
            if !self.buffer_out.is_empty() {
                return Ok(self.buffer_out.pop_front().unwrap());
            }
            while self.buffer_in.len() < self.rm.rule.len() {
                self.buffer_in.push_back(self.tokens.next_token(state)?)
            }
            let buf = self.buffer_in.make_contiguous();
            if self.rm.matches_rule(buf) {
                let toks_in: Vec<Token> = std::mem::replace(
                    &mut self.buffer_in,
                    VecDeque::with_capacity(self.rm.rule.len()),
                )
                .into();
                let spos = toks_in[0].pos.clone();
                self.buffer_out = match self.rm.apply_rule(toks_in, state) {
                    Ok(toks_out) => toks_out.into(),
                    Err(je) => return Err(TokenError::new(&format!("{}", je), spos)),
                };
            } else {
                return Ok(self.buffer_in.pop_front().unwrap());
            }
        }
    }
}
