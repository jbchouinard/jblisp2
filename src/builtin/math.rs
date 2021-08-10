use crate::builtin::get_n_plus_args;
use crate::*;

pub fn jbuiltin_add(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut acc: JTInt = 0;
    for arg in args.iter_list()? {
        acc += arg.to_int()?;
    }
    Ok(state.jint(acc))
}

pub fn jbuiltin_sub(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let ([init], rest) = get_n_plus_args(args)?;
    if rest.is_empty() {
        Ok(state.jint(-init.to_int()?))
    } else {
        let mut acc: JTInt = init.to_int()?;
        for arg in rest {
            acc -= arg.to_int()?
        }
        Ok(state.jint(acc))
    }
}

pub fn jbuiltin_mul(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut acc: JTInt = 1;
    for arg in args.iter_list()? {
        acc *= arg.to_int()?;
    }
    Ok(state.jint(acc))
}

pub fn jbuiltin_div(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let ([init], rest) = get_n_plus_args(args)?;
    if rest.is_empty() {
        Ok(state.jint(1 / init.to_int()?))
    } else {
        let mut acc: JTInt = init.to_int()?;
        for arg in rest {
            acc /= arg.to_int()?
        }
        Ok(state.jint(acc))
    }
}
