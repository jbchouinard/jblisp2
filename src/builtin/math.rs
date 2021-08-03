use crate::*;

pub fn jbuiltin_add(args: &JCell, _env: JEnvRef) -> JResult {
    let mut sum: JTInt = 0;
    for arg in args.iter()? {
        match arg.to_int() {
            Ok(n) => {
                sum += n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JValue::Int(sum).into_ref())
}

pub fn jbuiltin_mul(args: &JCell, _env: JEnvRef) -> JResult {
    let mut acc: JTInt = 1;
    for arg in args.iter()? {
        match arg.to_int() {
            Ok(n) => {
                acc *= n;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(JValue::Int(acc).into_ref())
}
