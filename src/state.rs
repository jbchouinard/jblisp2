use std::path::Path;
use std::rc::Rc;

use crate::error::JErrorKind;
use crate::primitives::intern::Interned;
use crate::reader::parser::Parser;
use crate::reader::tokenizer::TokenIter;
use crate::reader::tokenizer::Tokenizer;
use crate::reader::PositionTag;
use crate::*;

const STR_INTERN_MAX_LEN: usize = 1024;
const INT_INTERN_MAX_VAL: JTInt = 1024;

fn make_nil() -> JValRef {
    JVal::Nil.into_ref()
}
fn make_bool(b: bool) -> JValRef {
    JVal::Bool(b).into_ref()
}
fn make_int(n: JTInt) -> JValRef {
    JVal::Int(n).into_ref()
}
fn make_sym(s: String) -> JValRef {
    JVal::Symbol(s).into_ref()
}
fn make_str(s: String) -> JValRef {
    JVal::String(s).into_ref()
}

pub struct JState {
    const_nil: JValRef,
    const_true: JValRef,
    const_false: JValRef,
    interned_int: Interned<JTInt>,
    interned_sym: Interned<String>,
    interned_str: Interned<String>,
    pos: PositionTag,
    traceback: Vec<PositionTag>,
}

impl JState {
    pub fn new() -> Self {
        Self {
            const_nil: make_nil(),
            const_true: make_bool(true),
            const_false: make_bool(false),
            interned_int: Interned::new(Box::new(make_int)),
            interned_sym: Interned::new(Box::new(make_sym)),
            interned_str: Interned::new(Box::new(make_str)),
            pos: PositionTag {
                filename: "".to_string(),
                lineno: 0,
                col: 0,
            },
            traceback: vec![],
        }
    }
    pub fn update_pos(&mut self, pt: Option<&PositionTag>) {
        if let Some(pos) = pt {
            self.pos = pos.clone();
        }
    }
    pub fn traceback_push(&mut self, pt: &PositionTag) {
        self.traceback.push(pt.clone())
    }
    pub fn traceback_take(&mut self) -> Vec<PositionTag> {
        std::mem::take(&mut self.traceback)
    }
    pub fn traceback(&self) -> &[PositionTag] {
        &self.traceback
    }

    pub fn eval_tokens(
        &mut self,
        name: &str,
        tokeniter: Box<dyn TokenIter>,
        env: JEnvRef,
    ) -> Result<Option<JValRef>, (PositionTag, JError)> {
        let forms = match Parser::new(name, tokeniter, self).parse_forms() {
            Ok(forms) => forms,
            Err(pe) => return Err((pe.pos.clone(), pe.into())),
        };
        let mut last_eval = None;
        for (pos, expr) in forms {
            self.update_pos(Some(&pos));
            last_eval = match eval(expr, Rc::clone(&env), self) {
                Ok(val) => Some(val),
                Err(je) => return Err((self.pos.clone(), je)),
            }
        }
        Ok(last_eval)
    }
    pub fn eval_str(
        &mut self,
        name: &str,
        program: &str,
        env: JEnvRef,
    ) -> Result<Option<JValRef>, (PositionTag, JError)> {
        let tokeniter = Box::new(Tokenizer::new(program.to_string()));
        self.eval_tokens(name, tokeniter, env)
    }
    pub fn eval_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        env: JEnvRef,
    ) -> Result<Option<JValRef>, (PositionTag, JError)> {
        let path = path.as_ref();
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) => {
                return Err((
                    PositionTag::new("", 0, 0),
                    JError::new(JErrorKind::OsError, &format!("{}", e)),
                ))
            }
        };
        self.eval_str(&path.to_string_lossy(), &text, env)
    }

    // Constructors
    pub fn jnil(&self) -> JValRef {
        Rc::clone(&self.const_nil)
    }
    pub fn jbool(&self, val: bool) -> JValRef {
        if val {
            Rc::clone(&self.const_true)
        } else {
            Rc::clone(&self.const_false)
        }
    }
    pub fn jint(&mut self, val: JTInt) -> JValRef {
        if val > INT_INTERN_MAX_VAL {
            (self.interned_int.constructor)(val)
        } else {
            self.interned_int.get_or_insert(val)
        }
    }
    pub fn jsymbol(&mut self, val: String) -> JValRef {
        self.interned_sym.get_or_insert(val)
    }
    pub fn jstring(&mut self, val: String) -> JValRef {
        if val.len() > STR_INTERN_MAX_LEN {
            (self.interned_str.constructor)(val)
        } else {
            self.interned_str.get_or_insert(val)
        }
    }
    pub fn jquote(&self, v: JValRef) -> JValRef {
        JVal::Quote(v).into_ref()
    }
    pub fn jlist(&self, mut v: Vec<JValRef>) -> JValRef {
        let mut cur = self.jnil();
        v.reverse();
        for val in v {
            cur = self.jpair(val, cur);
        }
        cur
    }
    pub fn jpair(&self, left: JValRef, right: JValRef) -> JValRef {
        JVal::Pair(JPair::cons(left, right)).into_ref()
    }
    pub fn jerrorval(&self, kind: JErrorKind, reason: &str) -> JValRef {
        JVal::Error(JError::new(kind, reason)).into_ref()
    }
    pub fn jlambda(&mut self, clos: JEnvRef, params: Vec<String>, code: Vec<JValRef>) -> JResult {
        Ok(JVal::Lambda(Box::new(JLambda {
            closure: clos,
            params: JParams::new(params)?,
            code,
        }))
        .into_ref())
    }
    pub fn jmacro(&mut self, clos: JEnvRef, params: Vec<String>, code: Vec<JValRef>) -> JResult {
        Ok(JVal::Macro(Box::new(JLambda {
            closure: clos,
            params: JParams::new(params)?,
            code,
        }))
        .into_ref())
    }
    pub fn jbuiltin(
        &self,
        name: String,
        f: Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>,
    ) -> JValRef {
        JVal::Builtin(JBuiltin { name, f }).into_ref()
    }
    pub fn jspecialform(
        &self,
        name: String,
        f: Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>,
    ) -> JValRef {
        JVal::SpecialForm(JBuiltin { name, f }).into_ref()
    }
}

impl Default for JState {
    fn default() -> Self {
        Self::new()
    }
}
