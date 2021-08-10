use crate::*;

pub fn jbuiltin_concat(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut strings: Vec<String> = vec![];
    for arg in args.iter_list()? {
        strings.push(arg.to_str()?.to_owned())
    }
    Ok(state.jstring(strings.join("")))
}
