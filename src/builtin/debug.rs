use std::rc::Rc;

use crate::builtin::get_n_args;
use crate::*;

pub fn jbuiltin_display_debug(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    println!("{:?}", val);
    Ok(state.jnil())
}

pub fn jbuiltin_display_debug_pretty(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    println!("{:#?}", val);
    Ok(state.jnil())
}

pub fn jbuiltin_display_ptr(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    println!("{:p}", val);
    Ok(state.jnil())
}

pub fn jspecial_display_debug_macro(args: JValRef, env: JEnvRef, state: &mut JState) -> JResult {
    let args = args.to_pair()?;
    let first = eval(args.car(), env, state)?;
    let args = args.cdr();
    let lambda = match &*first {
        JVal::Macro(l) => l.clone(),
        _ => return Err(JError::new(TypeError, "expected a macro")),
    };
    let invoke_env = JEnv::new(Some(Rc::clone(&lambda.closure))).into_ref();
    lambda.params.bind(args, Rc::clone(&invoke_env))?;
    let mut last_res = state.jnil();
    for expr in &lambda.code {
        last_res = eval(Rc::clone(expr), Rc::clone(&invoke_env), state)?;
    }
    println!("{}", last_res);
    Ok(state.jnil())
}

pub fn jbuiltin_display_code(args: JValRef, _env: JEnvRef, state: &mut JState) -> JResult {
    let [val] = get_n_args(args)?;
    let (t, params, code) = match &*val {
        JVal::Lambda(jl) => ("fn".to_string(), &jl.params, &jl.code),
        JVal::Macro(jl) => ("fn".to_string(), &jl.params, &jl.code),
        _ => {
            return Err(JError::new(
                TypeError,
                "expected non-builtin lambda or macro",
            ))
        }
    };
    println!(
        "({} {} {})",
        t,
        params,
        code.iter()
            .map(|v| repr(v))
            .collect::<Vec<String>>()
            .join(" ")
    );
    Ok(state.jnil())
}
