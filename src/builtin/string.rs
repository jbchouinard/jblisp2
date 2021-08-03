use crate::*;

pub fn jbuiltin_concat(args: &JCell, _env: JEnvRef) -> JResult {
    let mut strings: Vec<String> = vec![];
    for arg in args.iter().unwrap() {
        match &*arg {
            JValue::String(s) => strings.push(s.clone()),
            _ => return Err(JError::new("TypeError", "expected strings")),
        }
    }
    Ok(JValue::String(strings.join("")).into_ref())
}
