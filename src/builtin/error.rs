use crate::builtin::*;

pub fn jbuiltin_exception(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [emsg] = get_n_args(args)?;
    let emsg = emsg.to_str()?;
    Ok(state.error(Exception, emsg))
}

pub fn jbuiltin_error(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [etype, emsg] = get_n_args(args)?;
    let etype = etype.to_symbol()?;
    let emsg = emsg.to_str()?;
    Ok(state.error(UserDefined(etype.to_string()), emsg))
}

pub fn jbuiltin_raise(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [err] = get_n_args(args)?;
    Err(err.to_error()?.clone())
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
            errenv.define("err", JVal::Error(je).into_ref());
            eval(except, errenv.into_ref(), state)
        }
    }
}
