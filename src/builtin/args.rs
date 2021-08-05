use std::convert::TryInto;

use crate::*;

pub fn get_n_args<const N: usize>(args: JValRef) -> Result<[JValRef; N], JError> {
    let args: Vec<JValRef> = args.iter_list()?.collect();
    if args.len() == N {
        Ok(args.try_into().unwrap())
    } else {
        Err(JError::new(
            "ArgumentError",
            &format!("expected {} arguments", N),
        ))
    }
}
