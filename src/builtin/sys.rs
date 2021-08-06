use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_getenv(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [var] = get_n_args(args)?;
    match &*var {
        JVal::String(s) => match std::env::var(s) {
            Ok(val) => Ok(state.string(val)),
            Err(e) => Err(JError::OsError(format!("{}", e))),
        },
        _ => Err(JError::TypeError("expected string".to_string())),
    }
}
