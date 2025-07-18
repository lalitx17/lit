use crate::utils::last_commit_hash;
use std::fs;
use std::fs::File;
use std::io::{Result, Write};

pub fn checkout(hash: Option<String>, branch: Option<String>, new_branch: bool) -> Result<()> {
    if new_branch == true && hash.is_some() {
    } else if new_branch == true {
        if let Some(branch_name) = branch {
            create_and_switch_branch(branch_name)?;
        }
    } else if hash.is_some() {
    } else {
    }
    Ok(())
}

pub fn create_and_switch_branch(branch: String) -> Result<()> {
    let last_commit_hash = last_commit_hash()?;

    let head_content = format!("ref: refs/heads/{}", branch);
    fs::write(".lit/HEAD", head_content)?;

    let branch_file = format!(".lit/refs/heads/{}", branch);
    let mut file = File::create(branch_file)?;

    file.write_all(last_commit_hash.as_bytes())?;

    Ok(())
}

/* pub fn switch_commit(hash: &String) -> Result<()> {
    Ok(())
} */
