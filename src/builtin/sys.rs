use std::convert::TryInto;

use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_get_env_var(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [var] = get_n_args(args)?;
    let var = var.to_str()?;
    match std::env::var(var) {
        Ok(val) => Ok(state.string(val)),
        Err(e) => Err(JError::new(OsError, &format!("{}", e))),
    }
}

pub fn jbuiltin_exit(args: JValRef, _env: JEnvRef, _state: &mut JState) -> JResult {
    let [exitcode] = get_n_args(args)?;
    std::process::exit(exitcode.to_int()?.try_into().unwrap());
}

pub fn jbuiltin_paths(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [] = get_n_args(args)?;
    for p in crate::import::JIBI_PATHS.iter() {
        println!("{}", p.display());
    }
    Ok(state.nil())
}
