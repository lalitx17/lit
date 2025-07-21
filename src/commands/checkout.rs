use crate::commands::show::{ShowResult, show};
use crate::utils::{does_branch_exists, is_lit_initialized, last_commit_hash};
use std::fs;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn checkout(hash: Option<String>, new_branch: bool, branch: Option<String>) -> Result<()> {
    is_lit_initialized()?;
    if new_branch == true && hash.is_some() {
        if let Some(commit_hash) = hash {
            switch_commit(commit_hash)?;
        }
        if let Some(branch_name) = branch {
            create_and_switch_branch(branch_name)?;
        }
    } else if new_branch == true {
        if let Some(branch_name) = branch {
            create_and_switch_branch(branch_name)?;
        }
    } else if hash.is_some() {
        if let Some(commit_hash) = hash {
            switch_commit(commit_hash)?;
        }
    } else {
        println!("Missing Arguments!!!")
    }
    Ok(())
}

pub fn create_and_switch_branch(branch: String) -> Result<()> {
    let branch_exists = does_branch_exists(&branch)?;
    if branch_exists {
        println!("{} branch already exists.", branch);
    }
    let last_commit_hash = last_commit_hash()?;

    let head_content = format!("ref: refs/heads/{}", branch);
    fs::write(".lit/HEAD", head_content)?;

    let branch_file = format!(".lit/refs/heads/{}", branch);
    let mut file = File::create(branch_file)?;

    file.write_all(last_commit_hash.as_bytes())?;

    Ok(())
}

pub fn switch_commit(hash: String) -> Result<()> {
    match show(&hash)? {
        ShowResult::NotFound => {
            println!("hash doesn't exist");
            return Ok(());
        }
        ShowResult::Exists(content) => {
            println!("hash exists");
            let tree_line = content.lines().find(|l| l.starts_with("tree "));
            let tree_hash = match tree_line {
                Some(line) => line[5..].trim(),
                None => return Ok(()),
            };
            clean_working_dir(Path::new("."))?;
            restore_tree(tree_hash, "")?;
            write_hash_to_current_branch(&hash)?;
        }
    }
    Ok(())
}

pub fn restore_tree(tree_hash: &str, path_prefix: &str) -> Result<()> {
    let tree_data = match show(&tree_hash.to_string())? {
        ShowResult::NotFound => {
            println!("tree hash doesn't exist: {}", tree_hash);
            return Ok(());
        }
        ShowResult::Exists(data) => data,
    };

    let mut i = 0;
    let tree_bytes = tree_data.as_bytes();

    while i < tree_bytes.len() {
        let null_pos = tree_bytes[i..].iter().position(|&b| b == 0).unwrap();
        let header = std::str::from_utf8(&tree_bytes[i..i + null_pos]).unwrap();

        i += null_pos + 1;

        let hash = std::str::from_utf8(&tree_bytes[i..i + 40]).unwrap();
        i += 40;

        if i < tree_bytes.len() && tree_bytes[i] == b'\n' {
            i += 1;
        }

        let mut parts = header.splitn(2, ' ');
        let object_type = parts.next().unwrap();
        let name = parts.next().unwrap().trim();

        let file_path = format!("{}{}", path_prefix, name);

        if object_type == "blob" {
            let blob_content = match show(&hash.to_string())? {
                ShowResult::NotFound => {
                    println!("blob hash doesn't exist: {}", hash);
                    continue;
                }
                ShowResult::Exists(data) => data,
            };
            if let Some(parent) = Path::new(&file_path).parent() {
                fs::create_dir_all(parent)?;
            }

            std::fs::write(&file_path, blob_content)?;
        } else if object_type == "tree" {
            fs::create_dir_all(&file_path)?;
            let subdir_prefix = format!("{}{}/", path_prefix, name);
            restore_tree(hash, &subdir_prefix)?;
        }
    }

    Ok(())
}

fn clean_working_dir(root: &Path) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == ".lit" || name == ".git" {
                continue;
            }
        }

        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn write_hash_to_current_branch(hash: &str) -> Result<()> {
    let head_content = std::fs::read_to_string(".lit/HEAD")?;
    let branch_ref = head_content.trim().strip_prefix("ref: ").unwrap();

    let location = format!(".lit/{}", branch_ref);
    std::fs::write(location, hash)?;
    Ok(())
}
