use std::fs;
use std::io::Result;

pub fn branch_list() -> Result<()> {
    let heads_dir = ".lit/refs/heads";
    let entries = fs::read_dir(heads_dir)?;
    let mut branches = Vec::new();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap_or_default();
        branches.push(name);
    }

    let head_content = fs::read_to_string(".lit/HEAD")?;
    let current_branch = head_content
        .trim()
        .strip_prefix("ref: refs/heads/")
        .unwrap_or("");

    for branch in branches {
        if branch == current_branch {
            println!("*\x1b[1m{}\x1b[0m", branch);
        } else {
            println!("{}", branch);
        }
    }
    Ok(())
}
