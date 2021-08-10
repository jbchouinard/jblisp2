use std::convert::TryInto;

use crate::*;

pub fn get_n_args<const N: usize>(args: JValRef) -> Result<[JValRef; N], JError> {
    let args: Vec<JValRef> = args.iter_list()?.collect();
    if args.len() == N {
        Ok(args.try_into().unwrap())
    } else {
        Err(JError::new(
            ApplyError,
            &format!("expected {} argument(s)", N),
        ))
    }
}

pub fn get_n_plus_args<const N: usize>(
    args: JValRef,
) -> Result<([JValRef; N], Vec<JValRef>), JError> {
    let mut args: Vec<JValRef> = args.iter_list()?.collect();
    if args.len() >= N {
        let rest = args.split_off(N);
        Ok((args.try_into().unwrap(), rest))
    } else {
        Err(JError::new(
            ApplyError,
            &format!("expected at least {} argument(s)", N),
        ))
    }
}
