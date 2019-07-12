use std::env;
use tico::tico;
use colored::*;

pub fn cwd() -> Option<colored::ColoredString> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("EXPAND_TILDE").unwrap_or("0".into());

    match tilde_expand.as_ref() {
        "0" => {
            if (&path[..]).starts_with(&home[..]) && home.len() > 0 {
                path = path.replacen(&home[..], "~", 1);
            }
        }
        _ => {}
    };

    let cwd_shorten = env::var("SHORTEN_CWD").unwrap_or("1".into());
    let cwd_color = env::var("CWD_COLOR").unwrap_or("white".into());
    match cwd_shorten.as_ref() {
        "0" => return Some(path.color(cwd_color)),
        _ => return Some(tico(&path[..]).color(cwd_color))
    }

}
