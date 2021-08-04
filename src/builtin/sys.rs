use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_getenv(args: &JCell, _env: JEnvRef) -> JResult {
    let [var] = get_n_args(args)?;
    match &*var {
        JValue::String(s) => match std::env::var(s) {
            Ok(val) => Ok(JValue::String(val).into_ref()),
            Err(e) => Err(JError::new("VarError", &format!("{}", e))),
        },
        _ => Err(JError::new("TypeError", "expected string")),
    }
}
