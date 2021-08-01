use super::tokenizer::{Token, TokenValue, Tokenizer};
use super::Result;
use crate::*;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokenizer = Tokenizer::new(input);
        Self {
            tokenizer,
            // Dummy value until we read the first real token
            peek: Token::new(TokenValue::Whitespace, 0),
        }
    }

    fn next(&mut self) -> Result<Token> {
        let next: Token = self.tokenizer.next_token()?;
        let cur = std::mem::replace(&mut self.peek, next);
        Ok(cur)
    }

    fn expect(&mut self, tok: TokenValue) -> Result<Token> {
        let next = self.next()?;
        if next.value == tok {
            Ok(next)
        } else {
            Err(ReaderError::new(
                &format!("expected token {:?}", tok),
                next.pos,
            ))
        }
    }

    fn eat(&mut self, tok: TokenValue) -> Result<()> {
        while self.peek.value == tok {
            self.next()?;
        }
        Ok(())
    }

    fn expr(&mut self) -> Result<JValue> {
        self.eat(TokenValue::Whitespace)?;
        match self.peek.value {
            TokenValue::LParen => self.sexpr(),
            _ => self.atom(),
        }
    }

    fn atom(&mut self) -> Result<JValue> {
        let next = self.next()?;
        match next.value {
            TokenValue::Int(n) => Ok(JValue::Int(n)),
            TokenValue::Ident(s) => Ok(JValue::Symbol(s)),
            _ => Err(ReaderError::new(
                &format!("expected int or symbol, got {:?}", next.value),
                next.pos,
            )),
        }
    }

    fn sexpr(&mut self) -> Result<JValue> {
        self.expect(TokenValue::LParen)?;
        self.eat(TokenValue::Whitespace)?;
        let mut list = vec![];
        while self.peek.value != TokenValue::RParen {
            list.push(self.expr()?);
            self.eat(TokenValue::Whitespace)?;
        }
        self.expect(TokenValue::RParen)?;
        Ok(JValue::SExpr(list))
    }

    pub fn parse(&mut self) -> JValue {
        if let Err(re) = self.next() {
            return JValue::Error(re.into());
        }
        match self.expr() {
            Ok(val) => val,
            Err(re) => JValue::Error(re.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_1() {
        let mut parser = Parser::new("(+ 12 -15)");
        let val = parser.parse();

        assert_eq!(val, jsexpr![jsym("+"), jint(12), jint(-15)]);
    }

    #[test]
    fn test_parser_2() {
        let mut parser = Parser::new("(* (+ 12 -33) 42)");
        let val = parser.parse();

        assert_eq!(
            val,
            jsexpr![jsym("*"), jsexpr![jsym("+"), jint(12), jint(-33)], jint(42)]
        );
    }
}
