use crate::commands::show::{ShowResult, show};
use crate::utils::last_commit_hash;
use std::io::Result;

pub fn log() -> Result<String> {
    let mut commit_hash = last_commit_hash()?;
    let mut log_output = String::new();

    while !commit_hash.is_empty() {
        match show(&commit_hash)? {
            ShowResult::NotFound => {
                log_output.push_str(&format!("commit {} not found\n---\n", commit_hash));
                break;
            }
            ShowResult::Exists(commit_data) => {
                log_output.push_str(&commit_data);
                log_output.push_str("\n---\n");
                let parent_line = commit_data.lines().find(|l| l.starts_with("parent "));
                if let Some(line) = parent_line {
                    commit_hash = line[7..].trim().to_string();
                } else {
                    break;
                }
            }
        }
    }

    Ok(log_output)
}
