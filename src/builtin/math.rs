use crate::*;

pub fn jbuiltin_add(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut sum: JTInt = 0;
    for arg in args.iter_list()? {
        match arg.to_int() {
            Ok(n) => {
                sum += n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JVal::int(sum, state))
}

pub fn jbuiltin_mul(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let mut acc: JTInt = 1;
    for arg in args.iter_list()? {
        match arg.to_int() {
            Ok(n) => {
                acc *= n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JVal::int(acc, state))
}
