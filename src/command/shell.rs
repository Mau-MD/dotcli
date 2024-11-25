use std::{env, fs, path::Path};

pub fn find_shell_config_path() -> Result<String, String> {
    let possible_paths = vec!["~/.zprofile", "~/.zshrc", "~/.bashrc", "~/.bash_profile"]
        .iter()
        .map(|p| tilde_to_home(p))
        .collect::<Result<Vec<String>, String>>()?;

    for path in possible_paths {
        if fs::exists(&path).is_ok() {
            return Ok(path);
        }
    }
    Err("No shell config found".to_string())
}

pub fn is_absolute_path(path: &str) -> bool {
    Path::new(path).is_absolute()
}

fn tilde_to_home(path: &str) -> Result<String, String> {
    if path.starts_with("~") {
        Ok(path.replace("~", &env::var("HOME").unwrap()))
    } else {
        Ok(path.to_string())
    }
}
