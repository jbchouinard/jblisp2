use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_getenv(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [var] = get_n_args(args)?;
    match &*var {
        JVal::String(s) => match std::env::var(s) {
            Ok(val) => Ok(JVal::str(val, state)),
            Err(e) => Err(JError::new("VarError", &format!("{}", e))),
        },
        _ => Err(JError::new("TypeError", "expected string")),
    }
}
