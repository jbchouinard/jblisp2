use crate::builtin::Args;
use crate::*;

pub fn jbuiltin_add(args: Args, _env: JEnvRef) -> JResult {
    let mut sum: JTInt = 0;
    for arg in args {
        match arg.to_int() {
            Ok(n) => {
                sum += n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JValue::Int(sum).into_ref())
}

pub fn jbuiltin_mul(args: Args, _env: JEnvRef) -> JResult {
    let mut sum: JTInt = 0;
    for arg in args {
        match arg.to_int() {
            Ok(n) => {
                sum *= n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JValue::Int(sum).into_ref())
}
