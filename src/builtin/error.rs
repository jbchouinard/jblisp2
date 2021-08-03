use crate::builtin::*;

pub fn jbuiltin_error(args: &JCell, _env: JEnvRef) -> JResult {
    let [emsg] = get_n_args(args)?;
    match &*emsg {
        JValue::String(s) => Ok(JValue::Error(JError::new("Error", s)).into_ref()),
        _ => Err(JError::new("TypeError", "expected a string")),
    }
}

pub fn jbuiltin_raise(args: &JCell, _env: JEnvRef) -> JResult {
    let [err] = get_n_args(args)?;
    match &*err {
        JValue::Error(je) => Err(je.clone()),
        _ => Err(JError::new("TypeError", "expected an error")),
    }
}

// Error handling
// >>> (try (raise (error "foo")) "caught")
// "caught"
// >>> (try "no-error" "caught")
// "no-error"
pub fn jspecial_try(args: &JCell, env: JEnvRef) -> JResult {
    let [code, except] = get_n_args(args)?;
    match jeval(code, Rc::clone(&env)) {
        Ok(val) => Ok(val),
        Err(je) => {
            let errenv = JEnv::new(Some(env));
            errenv.define("err", JValue::Error(je).into_ref());
            jeval(except, errenv.into_ref())
        }
    }
}
