use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use lazy_static::lazy_static;

fn get_paths() -> Vec<PathBuf> {
    let mut paths = vec![];

    if let Ok(s) = env::var("JIBI_PATH") {
        for s in s.split(':') {
            if let Ok(path) = PathBuf::from_str(s).unwrap().canonicalize() {
                paths.push(path);
            }
        }
    }

    if let Some(prefix) = option_env!("PREFIX") {
        let mut prefix = PathBuf::from_str(prefix).unwrap();
        prefix.push("lib/jibi");
        if let Ok(path) = prefix.canonicalize() {
            paths.push(path);
        }
    }
    let here = env::current_dir().unwrap();
    if let Ok(path) = here.canonicalize() {
        paths.push(path);
    }
    paths
}

lazy_static! {
    pub static ref JIBI_PATHS: Vec<PathBuf> = get_paths();
}

pub fn find_module<P: AsRef<Path>>(p: P) -> Option<PathBuf> {
    let path: &Path = p.as_ref();
    if path.is_absolute() {
        if path.exists() {
            return Some(path.to_path_buf());
        }
    } else {
        for prefix in JIBI_PATHS.iter() {
            let mut fullpath = prefix.to_path_buf();
            fullpath.push(path);
            if let Ok(fullpath) = fullpath.canonicalize() {
                return Some(fullpath);
            }
        }
    }
    None
}
