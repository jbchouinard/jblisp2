use super::tokenizer::{Token, TokenValue};
use crate::reader::tokenizer::TokenIter;

use super::ParserError;
use crate::*;

pub struct Parser<'a> {
    filename: String,
    lineno: usize,
    last_newline_pos: usize,
    tokeniter: Box<dyn TokenIter>,
    peek: Token,
    state: &'a mut JState,
}

impl<'a> Parser<'a> {
    pub fn new(filename: &str, tokeniter: Box<dyn TokenIter>, state: &'a mut JState) -> Self {
        let mut this = Self {
            filename: filename.to_string(),
            lineno: 1,
            last_newline_pos: 0,
            tokeniter,
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

    fn _next(&mut self) -> Result<Token, ParserError> {
        let next = match self.tokeniter.next_token() {
            Ok(tok) => tok,
            Err(te) => return Err(self.error(te.pos, &te.reason)),
        };
        let cur = std::mem::replace(&mut self.peek, next);
        Ok(cur)
    }

    fn next(&mut self) -> Result<Token, ParserError> {
        let next = self._next()?;
        loop {
            match &self.peek.value {
                TokenValue::Comment(_) => {
                    self._next()?;
                }
                TokenValue::Whitespace(_) => {
                    self.whitespace()?;
                }
                _ => break,
            };
        }
        Ok(next)
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
            TokenValue::Int(n) => Ok(self.state.jint(n)),
            TokenValue::Ident(s) => Ok(self.state.jsymbol(s)),
            TokenValue::String(s) => Ok(self.state.jstring(s)),
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
        Ok(self.state.jlist(list))
    }

    pub fn parse_form(&mut self) -> Result<Option<JValRef>, ParserError> {
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
    use crate::reader::tokenizer::Tokenizer;

    fn test_parser(state: &mut JState, input: &str, expected: JValRef) {
        let mut parser = Parser::new("test", Box::new(Tokenizer::new(input.to_string())), state);
        let val = parser.expr().unwrap();
        assert_eq!(expected, val);
    }

    #[test]
    fn test_parser_1() {
        let mut state = JState::default();
        let lst = vec![
            state.jsymbol("+".to_string()),
            state.jint(12),
            state.jint(-15),
        ];
        let expected = state.jlist(lst);
        test_parser(&mut state, "(+ 12 -15)", expected);
    }

    #[test]
    fn test_parser_2() {
        let mut state = JState::default();
        let inner_lst = vec![
            state.jsymbol("+".to_string()),
            state.jint(12),
            state.jint(-33),
        ];
        let lst = vec![
            state.jsymbol("*".to_string()),
            state.jlist(inner_lst),
            state.jint(42),
        ];
        let expected = state.jlist(lst);
        test_parser(&mut state, "(* (+ 12 -33) 42)", expected)
    }

    #[test]
    fn test_parser_3() {
        let mut state = JState::default();
        let lst = vec![
            state.jsymbol("concat".to_string()),
            state.jstring("foo".to_string()),
            state.jstring("bar".to_string()),
        ];
        let expected = state.jlist(lst);
        test_parser(&mut state, "(concat \"foo\" \"bar\")", expected)
    }

    #[test]
    fn test_parser_4() {
        let mut state = JState::default();
        let inner_lst = vec![state.jint(1), state.jint(2), state.jint(3)];
        let lst = vec![
            state.jsymbol("quote".to_string()),
            state.jquote(state.jlist(inner_lst)),
        ];
        let expected = state.jlist(lst);
        test_parser(&mut state, "(quote '(1 2 3))", expected)
    }

    #[test]
    fn test_parser_5() {
        let mut state = JState::default();
        let inner_lst = vec![state.jint(1), state.jint(2), state.jint(3)];
        let lst = vec![
            state.jsymbol("quote".to_string()),
            state.jquote(state.jlist(inner_lst)),
        ];
        let expected = state.jlist(lst);
        test_parser(
            &mut state,
            "(quote ; this is a comment
            '(1 2 3))",
            expected,
        )
    }
}
