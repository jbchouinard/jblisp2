use crate::builtin::*;

pub fn jbuiltin_error(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [emsg] = get_n_args(args)?;
    match &*emsg {
        JVal::String(s) => Ok(state.error(JError::Exception(s.to_string()))),
        _ => Err(JError::TypeError("expected a string".to_string())),
    }
}

pub fn jbuiltin_raise(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [err] = get_n_args(args)?;
    match &*err {
        JVal::Error(je) => Err(je.clone()),
        _ => Err(JError::TypeError("expected an error".to_string())),
    }
}

// Error handling
// >>> (try (raise (error "foo")) "caught")
// "caught"
// >>> (try "no-error" "caught")
// "no-error"
pub fn jspecial_try(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let [code, except] = get_n_args(args)?;
    match eval(code, Rc::clone(&env), state) {
        Ok(val) => Ok(val),
        Err(je) => {
            let errenv = JEnv::new(Some(env));
            errenv.define("err", state.error(je));
            eval(except, errenv.into_ref(), state)
        }
    }
}
