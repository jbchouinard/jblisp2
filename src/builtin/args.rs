use std::convert::TryInto;

use crate::*;

pub type Args = Vec<JValueRef>;

pub fn get_n_args<const N: usize>(args: Args) -> Result<[JValueRef; N], JError> {
    if args.len() == N {
        Ok(args.try_into().unwrap())
    } else {
        Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", N),
        ))
    }
}
