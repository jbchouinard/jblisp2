use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::intern::Interned;
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
    traceback: Vec<TracebackFrame>,
    modules: HashMap<PathBuf, JEnvRef>,
    reader_macros: Vec<ReaderMacro>,
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
            modules: HashMap::new(),
            reader_macros: vec![],
        }
    }
    fn update_pos(&mut self, pt: Option<&PositionTag>) {
        if let Some(pos) = pt {
            self.pos = pos.clone();
        }
    }
    pub fn traceback_push(&mut self, tf: Option<TracebackFrame>) {
        if let Some(tf) = tf {
            self.traceback.push(tf)
        }
    }
    pub fn traceback_take(&mut self) -> Vec<TracebackFrame> {
        std::mem::take(&mut self.traceback)
    }
    pub fn traceback(&self) -> &[TracebackFrame] {
        &self.traceback
    }

    pub fn add_reader_macro(&mut self, rm: ReaderMacro) {
        self.reader_macros.push(rm);
    }
    pub fn reader_macros(&self) -> &Vec<ReaderMacro> {
        &self.reader_macros
    }

    pub fn import_module<P: AsRef<Path>>(&mut self, p: P, env: JEnvRef) -> JResult {
        let path = match std::fs::canonicalize(p) {
            Ok(p) => p,
            Err(e) => return Err(JError::new(OsError, &format!("{}", e))),
        };
        if self.modules.contains_key(&path) {
            return Ok(JVal::Env(Rc::clone(self.modules.get(&path).unwrap())).into_ref());
        }
        let modenv = JEnv::new(Some(Rc::clone(&env))).into_ref();
        if let Err((pos, err, _)) = self.eval_file(path.clone(), Rc::clone(&modenv)) {
            return Err(JError::new(EvalError, &format!("{}: {}", pos, err)));
        };
        self.modules.insert(path, Rc::clone(&modenv));
        Ok(JVal::Env(modenv).into_ref())
    }

    pub fn eval_tokens(
        &mut self,
        mut tokeniter: Box<dyn TokenProducer>,
        env: JEnvRef,
    ) -> Result<Option<JValRef>, (PositionTag, JError, Vec<TracebackFrame>)> {
        for rm in &self.reader_macros {
            tokeniter = Box::new(rm.apply(tokeniter));
        }
        let forms = match Parser::new(tokeniter, self).parse_forms() {
            Ok(forms) => forms,
            Err(pe) => return Err((pe.pos.clone(), pe.into(), self.traceback_take())),
        };
        let mut last_eval = None;
        for (pos, expr) in forms {
            self.update_pos(Some(&pos));
            last_eval = match eval(expr, Rc::clone(&env), self) {
                Ok(val) => Some(val),
                Err(je) => return Err((self.pos.clone(), je, self.traceback_take())),
            }
        }
        Ok(last_eval)
    }
    pub fn eval_str(
        &mut self,
        name: &str,
        program: &str,
        env: JEnvRef,
    ) -> Result<Option<JValRef>, (PositionTag, JError, Vec<TracebackFrame>)> {
        self.eval_tokens(
            Box::new(Tokenizer::new(name.to_string(), program.to_string())),
            env,
        )
    }
    pub fn eval_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        env: JEnvRef,
    ) -> Result<Option<JValRef>, (PositionTag, JError, Vec<TracebackFrame>)> {
        let path = path.as_ref();
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) => {
                return Err((
                    PositionTag::new("", 0, 0),
                    JError::new(JErrorKind::OsError, &format!("{}", e)),
                    self.traceback_take(),
                ))
            }
        };
        self.eval_str(&path.to_string_lossy(), &text, env)
    }

    // Constructors
    pub fn nil(&self) -> JValRef {
        Rc::clone(&self.const_nil)
    }
    pub fn bool(&self, val: bool) -> JValRef {
        if val {
            Rc::clone(&self.const_true)
        } else {
            Rc::clone(&self.const_false)
        }
    }
    pub fn int(&mut self, val: JTInt) -> JValRef {
        if val > INT_INTERN_MAX_VAL {
            (self.interned_int.constructor)(val)
        } else {
            self.interned_int.get_or_insert(val)
        }
    }
    pub fn float(&mut self, val: JTFloat) -> JValRef {
        JVal::Float(val).into_ref()
    }
    pub fn symbol(&mut self, val: String) -> JValRef {
        self.interned_sym.get_or_insert(val)
    }
    pub fn string(&mut self, val: String) -> JValRef {
        if val.len() > STR_INTERN_MAX_LEN {
            (self.interned_str.constructor)(val)
        } else {
            self.interned_str.get_or_insert(val)
        }
    }
    pub fn quote(&self, v: JValRef) -> JValRef {
        JVal::Quote(v).into_ref()
    }
    pub fn list(&self, mut v: Vec<JValRef>) -> JValRef {
        let mut cur = self.nil();
        v.reverse();
        for val in v {
            cur = self.pair(val, cur);
        }
        cur
    }
    pub fn pair(&self, left: JValRef, right: JValRef) -> JValRef {
        JVal::Pair(JPair::cons(left, right)).into_ref()
    }
    pub fn error(&self, kind: JErrorKind, reason: &str) -> JValRef {
        JVal::Error(JError::new(kind, reason)).into_ref()
    }
    pub fn lambda(
        &mut self,
        clos: JEnvRef,
        params: Vec<String>,
        code: Vec<JValRef>,
        name: Option<String>,
    ) -> JResult {
        Ok(JVal::Lambda(Box::new(JLambda {
            closure: clos,
            params: JParams::new(params)?,
            code,
            defpos: Some(self.pos.clone()),
            name,
        }))
        .into_ref())
    }
    pub fn r#macro(
        &mut self,
        clos: JEnvRef,
        params: Vec<String>,
        code: Vec<JValRef>,
        name: Option<String>,
    ) -> JResult {
        Ok(JVal::Macro(Box::new(JLambda {
            closure: clos,
            params: JParams::new(params)?,
            code,
            defpos: Some(self.pos.clone()),
            name,
        }))
        .into_ref())
    }
    pub fn builtin(
        &self,
        name: String,
        f: Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>,
    ) -> JValRef {
        JVal::Builtin(JBuiltin::new(name, f)).into_ref()
    }
    pub fn specialform(
        &self,
        name: String,
        f: Rc<dyn Fn(JValRef, JEnvRef, &mut JState) -> JResult>,
    ) -> JValRef {
        JVal::SpecialForm(JBuiltin::new(name, f)).into_ref()
    }
    pub fn token(&self, v: TokenValue) -> JResult {
        Ok(JVal::Token(Token::new(v, self.pos.clone())).into_ref())
    }
}

impl Default for JState {
    fn default() -> Self {
        Self::new()
    }
}
