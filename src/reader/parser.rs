use super::tokenizer::{Token, TokenValue, Tokenizer};
use super::Result;
use crate::types::vec_to_list;
use crate::*;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let tokenizer = Tokenizer::new(input);
        let mut this = Self {
            tokenizer,
            // Dummy value until we read the first real token
            peek: Token::new(TokenValue::Whitespace, 0),
        };
        this.next().unwrap();
        this
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
                &format!("expected token {:?}, got {:?}", tok, next.value),
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

    fn expr(&mut self) -> Result<JValueRef> {
        self.eat(TokenValue::Whitespace)?;
        match self.peek.value {
            TokenValue::LParen => self.sexpr(),
            TokenValue::Quote => self.quoted(),
            _ => self.atom(),
        }
    }

    fn quoted(&mut self) -> Result<JValueRef> {
        self.expect(TokenValue::Quote)?;
        Ok(JValue::Quoted(self.expr()?).into_ref())
    }

    fn atom(&mut self) -> Result<JValueRef> {
        let next = self.next()?;
        match next.value {
            TokenValue::Int(n) => Ok(JValue::Int(n).into_ref()),
            TokenValue::Ident(s) => Ok(JValue::Symbol(s).into_ref()),
            TokenValue::String(s) => Ok(JValue::String(s).into_ref()),
            _ => Err(ReaderError::new(
                &format!("unexpected token {:?}", next.value),
                next.pos,
            )),
        }
    }

    fn sexpr(&mut self) -> Result<JValueRef> {
        self.expect(TokenValue::LParen)?;
        self.eat(TokenValue::Whitespace)?;
        let mut list = vec![];
        while self.peek.value != TokenValue::RParen {
            list.push(self.expr()?);
            self.eat(TokenValue::Whitespace)?;
        }
        self.expect(TokenValue::RParen)?;
        Ok(JValue::Cell(vec_to_list(list)).into_ref())
    }

    pub fn parse_form(&mut self) -> Result<Option<JValueRef>> {
        self.eat(TokenValue::Whitespace)?;
        if self.peek.value == TokenValue::Eof {
            return Ok(None);
        }
        match self.expr() {
            Ok(val) => Ok(Some(val)),
            Err(e) => Err(e),
        }
    }

    pub fn parse_forms(&mut self) -> Result<Vec<JValueRef>> {
        let mut forms = vec![];
        while let Some(form) = self.parse_form()? {
            forms.push(form)
        }
        Ok(forms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_1() {
        let mut parser = Parser::new("(+ 12 -15)");
        let val = parser.expr().unwrap();

        assert_eq!(val, jsexpr![jsym("+"), jint(12), jint(-15)]);
    }

    #[test]
    fn test_parser_2() {
        let mut parser = Parser::new("(* (+ 12 -33) 42)");
        let val = parser.parse_form().unwrap().unwrap();

        assert_eq!(
            val,
            jsexpr![jsym("*"), jsexpr![jsym("+"), jint(12), jint(-33)], jint(42)]
        );
    }

    #[test]
    fn test_parser_3() {
        let mut parser = Parser::new("(concat \"foo\" \"bar\")");
        let val = parser.parse_form().unwrap().unwrap();

        assert_eq!(val, jsexpr![jsym("concat"), jstr("foo"), jstr("bar")]);
    }

    #[test]
    fn test_parser_4() {
        let mut parser = Parser::new("(quote '(1 2 3))");
        let val = parser.parse_form().unwrap().unwrap();

        assert_eq!(
            val,
            jsexpr![jsym("quote"), jquote(jsexpr![jint(1), jint(2), jint(3)])]
        )
    }
}
