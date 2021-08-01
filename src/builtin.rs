use std::rc::Rc;

use crate::*;

fn jbuiltin_add(args: Vec<JValue>) -> JValue {
    let mut sum: JTInt = 0;
    for arg in args {
        match arg.into_int() {
            Ok(n) => {
                sum += n;
            }
            Err(e) => return JValue::Error(e),
        }
    }
    JValue::Int(sum)
}

fn add_builtin<T>(name: &str, f: T, env: &mut JEnv)
where
    T: 'static + Fn(Vec<JValue>) -> JValue,
{
    let val = JValue::Builtin(JBuiltin {
        name: name.to_string(),
        f: Rc::new(f),
    });
    env.set(name, val);
}

pub fn add_builtins(env: &mut JEnv) {
    add_builtin("+", jbuiltin_add, env);
}
