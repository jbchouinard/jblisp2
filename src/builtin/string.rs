use crate::*;

pub fn jbuiltin_concat(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut strings: Vec<String> = vec![];
    for arg in args.iter_list().unwrap() {
        match &*arg {
            JVal::String(s) => strings.push(s.clone()),
            _ => return Err(JError::TypeError("expected strings".to_string())),
        }
    }
    Ok(state.str(strings.join("")))
}
