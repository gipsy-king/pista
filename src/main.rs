use std::env;
use tico::tico;
use git2::{ Repository, Status };
use colored::*;

fn main() {
    print!("{}", cwd());
    let (branch, status) = vcs_status().unwrap_or(("".into(), "".into()));
    println!(" {} {}", branch, status.dimmed());
    print!("{} ", prompt_char());
}

fn cwd() -> colored::ColoredString {
    let mut path = env::var("PWD").unwrap();
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("EXPAND_TILDE").unwrap_or("0".into());

    match tilde_expand.as_ref() {
        "0" => {},
        _ => path = path.replace(&home[..], "~")
    };

    let cwd_shorten = env::var("SHORTEN_CWD").unwrap_or("1".into());
    let cwd_color = env::var("CWD_COLOR").unwrap_or("white".into());
    match cwd_shorten.as_ref() {
        "0" => return path.color(cwd_color),
        _ => return tico(&path[..]).color(cwd_color)
    }

}

fn prompt_char() -> colored::ColoredString {
    let user_char = env::var("PROMPT_CHAR").unwrap_or("$ ".into());
    let root_char = env::var("PROMPT_CHAR_ROOT").unwrap_or("# ".into());

    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return root_char.red(),
        _ => return user_char.green()
    }
}

fn vcs_status() -> Option<(colored::ColoredString, colored::ColoredString)> {
    let current_dir = env::var("PWD").unwrap();

    let repo = match Repository::open(current_dir) {
        Ok(r) => r,
        Err(_) => return None
    };

    let reference = repo.head().unwrap();
    let mut branch;

    if reference.is_branch() {
        branch = format!("{}", reference.shorthand().unwrap()).bright_black();
    } else {
        let commit = reference.peel_to_commit().unwrap();
        let id = commit.id();
        branch = format!("{:.6}", id).bright_black();
    }

    let mut repo_stat = "".white();
    let git_clean_color          = env::var("GIT_CLEAN_COLOR").unwrap_or("green".into());
    let git_wt_modified_color    = env::var("GIT_WT_MODIFIED_COLOR").unwrap_or("red".into());
    let git_index_modified_color = env::var("GIT_INDEX_MODIFIED_COLOR").unwrap_or("yellow".into());

    let file_stats = repo.statuses(None).unwrap();
    for file in file_stats.iter() {
        match file.status() {
            // STATE: unstaged (working tree modified)
            Status::WT_NEW        | Status::WT_MODIFIED      |
            Status::WT_DELETED    | Status::WT_TYPECHANGE    |
            Status::WT_RENAMED => {
                let stat_char = env::var("GIT_WT_MODIFIED").unwrap_or("×".into());
                repo_stat = stat_char.color(&git_wt_modified_color[..]);
                break;
            },
            // STATE: staged (changes added to index)
            Status::INDEX_NEW     | Status::INDEX_MODIFIED   |
            Status::INDEX_DELETED | Status::INDEX_TYPECHANGE |
            Status::INDEX_RENAMED => {
                let stat_char = env::var("GIT_INDEX_MODIFIED").unwrap_or("±".into());
                repo_stat = stat_char.color(&git_index_modified_color[..]);
            },
            // STATE: comitted (changes have been saved in the repo)
            _ => {
                let stat_char = env::var("GIT_CLEAN").unwrap_or("·".into());
                repo_stat = stat_char.color(&git_clean_color[..]);
            }
        }
    }
    return Some((branch, repo_stat))
}
