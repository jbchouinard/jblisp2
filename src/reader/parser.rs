use crate::reader::ParserError;
use crate::*;

pub struct Parser<'a> {
    tokeniter: Box<dyn TokenProducer>,
    peek: Token,
    state: &'a mut JState,
}

impl<'a> Parser<'a> {
    pub fn new(tokeniter: Box<dyn TokenProducer>, state: &'a mut JState) -> Self {
        let mut this = Self {
            tokeniter,
            // Dummy value until we read the first real token
            peek: Token::new(TokenValue::Eof, PositionTag::new("", 0, 0)),
            state,
        };
        this.next().unwrap();
        this
    }

    fn error(&self, pos: PositionTag, reason: &str) -> ParserError {
        ParserError::new(pos, reason)
    }

    fn next(&mut self) -> Result<Token, ParserError> {
        let next = match self.tokeniter.next_token(self.state) {
            Ok(tok) => tok,
            Err(te) => return Err(self.error(te.pos, &te.reason)),
        };
        let cur = std::mem::replace(&mut self.peek, next);
        Ok(cur)
    }

    fn expect(&mut self, tok: TokenValue) -> Result<Token, ParserError> {
        let next = self.next()?;
        if next.value == tok {
            Ok(next)
        } else {
            Err(self.error(
                next.pos,
                &format!("expected token {:?}, got {:?}", tok, next.value),
            ))
        }
    }

    fn expr(&mut self) -> Result<JValRef, ParserError> {
        match self.peek.value {
            TokenValue::LParen => self.sexpr(),
            TokenValue::Quote => self.quote(),
            _ => self.atom(),
        }
    }

    fn quote(&mut self) -> Result<JValRef, ParserError> {
        self.expect(TokenValue::Quote)?;
        Ok(JVal::Quote(self.expr()?).into_ref())
    }

    fn atom(&mut self) -> Result<JValRef, ParserError> {
        let next = self.next()?;
        match next.value {
            TokenValue::Int(n) => Ok(self.state.int(n)),
            TokenValue::Ident(s) => Ok(self.state.symbol(s)),
            TokenValue::String(s) => Ok(self.state.string(s)),
            TokenValue::Float(x) => Ok(self.state.float(x)),
            _ => Err(self.error(next.pos, &format!("unexpected token {:?}", next.value))),
        }
    }

    fn sexpr(&mut self) -> Result<JValRef, ParserError> {
        self.expect(TokenValue::LParen)?;
        let mut list = vec![];
        while self.peek.value != TokenValue::RParen {
            list.push(self.expr()?);
        }
        self.expect(TokenValue::RParen)?;
        Ok(self.state.list(list))
    }

    pub fn parse_form(&mut self) -> Result<Option<(PositionTag, JValRef)>, ParserError> {
        if self.peek.value == TokenValue::Eof {
            return Ok(None);
        }
        let spos = self.peek.pos.clone();
        match self.expr() {
            Ok(val) => Ok(Some((spos, val))),
            Err(e) => Err(e),
        }
    }

    pub fn parse_forms(&mut self) -> Result<Vec<(PositionTag, JValRef)>, ParserError> {
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
    use crate::reader::tokenizer::Tokenizer;

    fn test_parser(state: &mut JState, input: &str, expected: JValRef) {
        let mut parser = Parser::new(
            Box::new(Tokenizer::new("test".to_string(), input.to_string())),
            state,
        );
        let val = parser.expr().unwrap();
        assert_eq!(expected, val);
    }

    #[test]
    fn test_parser_1() {
        let mut state = JState::default();
        let lst = vec![state.symbol("+".to_string()), state.int(12), state.int(-15)];
        let expected = state.list(lst);
        test_parser(&mut state, "(+ 12 -15)", expected);
    }

    #[test]
    fn test_parser_2() {
        let mut state = JState::default();
        let inner_lst = vec![state.symbol("+".to_string()), state.int(12), state.int(-33)];
        let lst = vec![
            state.symbol("*".to_string()),
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
            state.symbol("concat".to_string()),
            state.string("foo".to_string()),
            state.string("bar".to_string()),
        ];
        let expected = state.list(lst);
        test_parser(&mut state, "(concat \"foo\" \"bar\")", expected)
    }

    #[test]
    fn test_parser_4() {
        let mut state = JState::default();
        let inner_lst = vec![state.int(1), state.int(2), state.int(3)];
        let lst = vec![
            state.symbol("quote".to_string()),
            state.quote(state.list(inner_lst)),
        ];
        let expected = state.list(lst);
        test_parser(&mut state, "(quote '(1 2 3))", expected)
    }

    #[test]
    fn test_parser_5() {
        let mut state = JState::default();
        let inner_lst = vec![state.int(1), state.int(2), state.int(3)];
        let lst = vec![
            state.symbol("quote".to_string()),
            state.quote(state.list(inner_lst)),
        ];
        let expected = state.list(lst);
        test_parser(
            &mut state,
            "(quote ; this is a comment
            '(1 2 3))",
            expected,
        )
    }

    #[test]
    fn test_parser_6() {
        let mut state = JState::default();
        let lst = vec![
            state.symbol("+".to_string()),
            state.float(12.0),
            state.float(-0.5),
        ];
        let expected = state.list(lst);
        test_parser(&mut state, "(+ 12.0 -0.5)", expected);
    }

    #[test]
    fn test_parser_7() {
        let mut state = JState::default();
        let expected = state.float(2.025);
        test_parser(&mut state, "2.025", expected);
    }
}
