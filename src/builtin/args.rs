use std::convert::TryInto;

use crate::*;

pub fn get_n_args<const N: usize>(args: &JCell) -> Result<[JValueRef; N], JError> {
    let args: Vec<JValueRef> = args.iter()?.collect();
    if args.len() == N {
        Ok(args.try_into().unwrap())
    } else {
        Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", N),
        ))
    }
}
