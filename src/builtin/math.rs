use crate::*;

pub fn jbuiltin_add(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut acc: JTInt = 0;
    for arg in args.iter_list()? {
        acc += arg.to_int()?;
    }
    Ok(state.jint(acc))
}

// pub fn jbuiltin_sub(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
//     let argiter = args.iter_list()?;
//     let first = match argiter.next() {
//         Some(val) => val,
//         None => return Err(JError::new("ArgumentError", "expected at least 1 argument")),
//     };
//     let more_args = false;
// }

pub fn jbuiltin_mul(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut acc: JTInt = 1;
    for arg in args.iter_list()? {
        acc *= arg.to_int()?;
    }
    Ok(state.jint(acc))
}
