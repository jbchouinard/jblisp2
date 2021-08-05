use super::tokenizer::{Token, TokenValue, Tokenizer};
use super::Result;

use crate::reader::ReaderError;
use crate::*;

pub struct Parser<'a, 'b> {
    tokenizer: Tokenizer<'a>,
    peek: Token,
    state: &'b mut JState,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(input: &'a str, state: &'b mut JState) -> Self {
        let tokenizer = Tokenizer::new(input);
        let mut this = Self {
            tokenizer,
            // Dummy value until we read the first real token
            peek: Token::new(TokenValue::Whitespace, 0),
            state,
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

    fn expr(&mut self) -> Result<JValRef> {
        self.eat(TokenValue::Whitespace)?;
        match self.peek.value {
            TokenValue::LParen => self.sexpr(),
            TokenValue::Quote => self.quoted(),
            _ => self.atom(),
        }
    }

    fn quoted(&mut self) -> Result<JValRef> {
        self.expect(TokenValue::Quote)?;
        Ok(JVal::Quoted(self.expr()?).into_ref())
    }

    fn atom(&mut self) -> Result<JValRef> {
        let next = self.next()?;
        match next.value {
            TokenValue::Int(n) => Ok(self.state.int(n)),
            TokenValue::Ident(s) => Ok(self.state.sym(s)),
            TokenValue::String(s) => Ok(self.state.str(s)),
            _ => Err(ReaderError::new(
                &format!("unexpected token {:?}", next.value),
                next.pos,
            )),
        }
    }

    fn sexpr(&mut self) -> Result<JValRef> {
        self.expect(TokenValue::LParen)?;
        self.eat(TokenValue::Whitespace)?;
        let mut list = vec![];
        while self.peek.value != TokenValue::RParen {
            list.push(self.expr()?);
            self.eat(TokenValue::Whitespace)?;
        }
        self.expect(TokenValue::RParen)?;
        Ok(self.state.list(list))
    }

    pub fn parse_form(&mut self) -> Result<Option<JValRef>> {
        self.eat(TokenValue::Whitespace)?;
        if self.peek.value == TokenValue::Eof {
            return Ok(None);
        }
        match self.expr() {
            Ok(val) => Ok(Some(val)),
            Err(e) => Err(e),
        }
    }

    pub fn parse_forms(&mut self) -> Result<Vec<JValRef>> {
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

    fn test_parser(state: &mut JState, input: &str, expected: JValRef) {
        let mut parser = Parser::new(input, state);
        let val = parser.expr().unwrap();
        assert_eq!(expected, val);
    }

    #[test]
    fn test_parser_1() {
        let mut state = JState::default();
        let lst = vec![state.sym("+".to_string()), state.int(12), state.int(-15)];
        let expected = state.list(lst);
        test_parser(&mut state, "(+ 12 -15)", expected);
    }

    #[test]
    fn test_parser_2() {
        let mut state = JState::default();
        let inner_lst = vec![state.sym("+".to_string()), state.int(12), state.int(-33)];
        let lst = vec![
            state.sym("*".to_string()),
            state.list(inner_lst),
            state.int(42),
        ];
        let expected = state.list(lst);
        test_parser(&mut state, "(* (+ 12 -33) 42)", expected)
    }

    #[test]
    fn test_parser_3() {
        let mut state = JState::default();
        let lst = vec![
            state.sym("concat".to_string()),
            state.str("foo".to_string()),
            state.str("bar".to_string()),
        ];
        let expected = state.list(lst);
        test_parser(&mut state, "(concat \"foo\" \"bar\")", expected)
    }

    #[test]
    fn test_parser_4() {
        let mut state = JState::default();
        let inner_lst = vec![state.int(1), state.int(2), state.int(3)];
        let lst = vec![
            state.sym("quote".to_string()),
            state.quote(state.list(inner_lst)),
        ];
        let expected = state.list(lst);
        test_parser(&mut state, "(quote '(1 2 3))", expected)
    }
}
