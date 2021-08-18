use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

use crate::eval::apply_lambda;
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
    Char(char),
    Int(Matcher<JTInt>),
    Float(Matcher<JTFloat>),
    Ident(Matcher<String>),
    String(Matcher<String>),
    Eof,
    Or(Box<TokenMatcher>, Box<TokenMatcher>),
}

impl TokenMatcher {
    pub fn or(self, other: TokenMatcher) -> Self {
        TokenMatcher::Or(Box::new(self), Box::new(other))
    }
    pub fn matches(&self, tv: &TokenValue) -> bool {
        use TokenMatcher::*;
        match (self, tv) {
            (Or(tm1, tm2), _) => tm1.matches(tv) || tm2.matches(tv),
            (Any, TokenValue::Eof) => false,
            (Any, _) => true,
            (Eof, TokenValue::Eof) => true,
            (Char(c1), TokenValue::Char(c2)) => c1 == c2,
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
            Int(Matcher::Any) => write!(f, "INT(#ANY)"),
            Int(Matcher::Exact(n)) => write!(f, "INT({})", n),
            Float(Matcher::Any) => write!(f, "FLOAT(#ANY)"),
            Float(Matcher::Exact(x)) => write!(f, "FLOAT({})", x),
            Ident(Matcher::Exact(s)) => write!(f, "IDENT({})", s),
            Ident(Matcher::Any) => write!(f, "IDENT(#ANY)"),
            String(Matcher::Exact(s)) => write!(f, "STRING(\"{}\")", s),
            String(Matcher::Any) => write!(f, "String(#ANY)"),
            Eof => write!(f, "EOF"),
            Char(c) => write!(f, "CHAR('{}')", c),
            Or(tm1, tm2) => write!(f, "{}|{}", tm1, tm2),
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

pub struct LambdaTokenTransformer {
    lambda: JLambda,
    env: JEnvRef,
}

impl LambdaTokenTransformer {
    pub fn new(lambda: JLambda, env: JEnvRef) -> Self {
        Self { lambda, env }
    }
}

impl TokenTransformer for LambdaTokenTransformer {
    fn transform(&self, tokens: Vec<Token>, state: &mut JState) -> Result<Vec<Token>, JError> {
        apply_lambda(
            &self.lambda,
            state.list(
                tokens
                    .into_iter()
                    .map(|t| JVal::Token(t).into_ref())
                    .collect(),
            ),
            Rc::clone(&self.env),
            state,
        )?
        .iter_list()?
        .map(|v| v.to_token().map(|t| t.clone()))
        .collect()
    }
}

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
    pub fn apply(&self, tokens: Box<dyn TokenProducer>) -> ReaderMacroProducer {
        ReaderMacroProducer::new(tokens, self.clone())
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

pub struct ReaderMacroProducer {
    tokens: Box<dyn TokenProducer>,
    rm: ReaderMacro,
    buffer_in: VecDeque<Token>,
    buffer_out: VecDeque<Token>,
}

impl ReaderMacroProducer {
    pub fn new(tokens: Box<dyn TokenProducer>, rm: ReaderMacro) -> Self {
        let rl = rm.rule_len();
        Self {
            tokens,
            rm,
            buffer_in: VecDeque::with_capacity(rl),
            buffer_out: VecDeque::new(),
        }
    }
}

impl TokenProducer for ReaderMacroProducer {
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
