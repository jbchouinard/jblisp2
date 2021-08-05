use super::tokenizer::{Token, TokenValue, Tokenizer};

use super::ParserError;
use crate::*;

pub struct Parser<'a, 'b> {
    filename: String,
    lineno: usize,
    last_newline_pos: usize,
    tokenizer: Tokenizer<'a>,
    peek: Token,
    state: &'b mut JState,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(filename: &str, input: &'a str, state: &'b mut JState) -> Self {
        let tokenizer = Tokenizer::new(input);
        let mut this = Self {
            filename: filename.to_string(),
            lineno: 1,
            last_newline_pos: 0,
            tokenizer,
            // Dummy value until we read the first real token
            peek: Token::new(TokenValue::Whitespace("".to_string()), 0),
            state,
        };
        this.next().unwrap();
        this
    }

    fn error(&self, pos: usize, reason: &str) -> ParserError {
        ParserError::new(
            &self.filename,
            self.lineno,
            pos - self.last_newline_pos,
            reason,
        )
    }

    fn next(&mut self) -> Result<Token, ParserError> {
        let next = match self.tokenizer.next_token() {
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

    fn whitespace(&mut self) -> Result<(), ParserError> {
        while let TokenValue::Whitespace(ws) = &self.peek.value {
            let mut newline_count = 0;
            for (p, c) in ws.chars().enumerate() {
                if c == '\n' {
                    newline_count += 1;
                    self.last_newline_pos = self.peek.pos + p;
                }
            }
            self.lineno += newline_count;
            self.next()?;
        }
        Ok(())
    }

    fn expr(&mut self) -> Result<JValRef, ParserError> {
        self.whitespace()?;
        match self.peek.value {
            TokenValue::LParen => self.sexpr(),
            TokenValue::Quote => self.quoted(),
            _ => self.atom(),
        }
    }

    fn quoted(&mut self) -> Result<JValRef, ParserError> {
        self.expect(TokenValue::Quote)?;
        Ok(JVal::Quoted(self.expr()?).into_ref())
    }

    fn atom(&mut self) -> Result<JValRef, ParserError> {
        let next = self.next()?;
        match next.value {
            TokenValue::Int(n) => Ok(self.state.int(n)),
            TokenValue::Ident(s) => Ok(self.state.sym(s)),
            TokenValue::String(s) => Ok(self.state.str(s)),
            _ => Err(self.error(next.pos, &format!("unexpected token {:?}", next.value))),
        }
    }

    fn sexpr(&mut self) -> Result<JValRef, ParserError> {
        self.expect(TokenValue::LParen)?;
        self.whitespace()?;
        let mut list = vec![];
        while self.peek.value != TokenValue::RParen {
            list.push(self.expr()?);
            self.whitespace()?;
        }
        self.expect(TokenValue::RParen)?;
        Ok(self.state.list(list))
    }

    pub fn parse_form(&mut self) -> Result<Option<JValRef>, ParserError> {
        self.whitespace()?;
        if self.peek.value == TokenValue::Eof {
            return Ok(None);
        }
        match self.expr() {
            Ok(val) => Ok(Some(val)),
            Err(e) => Err(e),
        }
    }

    pub fn parse_forms(&mut self) -> Result<Vec<JValRef>, ParserError> {
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
        let mut parser = Parser::new("test", input, state);
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
