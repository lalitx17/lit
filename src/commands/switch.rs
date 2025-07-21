use crate::utils::{does_branch_exists, is_lit_initialized};
use std::fs;
use std::io::Result;

pub fn switch_branch(branch: String) -> Result<()> {
    is_lit_initialized()?;
    let branch_exists = does_branch_exists(&branch)?;
    if !branch_exists {
        println!("The given branch doesn't exists");
    } else {
        let branch = format!("ref: refs/heads/{}", branch);
        let location = format!(".lit/HEAD");
        fs::write(location, branch)?;
    }
    Ok(())
}
