use crate::commands::show;
use crate::utils::{is_lit_initialized, last_commit_hash};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Result, Write};

pub fn checkout(hash: Option<String>, branch: Option<String>, new_branch: bool) -> Result<()> {
    is_lit_initialized()?;
    if new_branch == true && hash.is_some() {
    } else if new_branch == true {
        if let Some(branch_name) = branch {
            create_and_switch_branch(branch_name)?;
        }
    } else if hash.is_some() {
        if let Some(commit_hash) = hash {
            switch_commit(commit_hash)?;
        }
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

pub fn switch_commit(hash: String) -> Result<()> {
    let content = show(&hash)?;

    let tree_line = content.lines().find(|l| l.starts_with("tree "));
    let tree_hash = match tree_line {
        Some(line) => line[5..].trim(),
        None => return Ok(()),
    };
    let mut tree_paths = HashSet::new();
    collect_tree_paths(tree_hash, "", &mut tree_paths)?;
    let root = std::path::Path::new(".");
    clean_working_dir(&tree_paths, root, root)?;
    restore_tree(tree_hash, "")?;
    Ok(())
}

pub fn restore_tree(tree_hash: &str, path_prefix: &str) -> Result<()> {
    let tree_data = show(&tree_hash.to_string())?;

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

        if object_type == "blob" {
            let blob_content = show(&hash.to_string())?;
            let file_path = format!("{}{}", path_prefix, name);
            std::fs::write(file_path, blob_content)?;
        } else if object_type == "tree" {
            let subdir_prefix = format!("{}{}/", path_prefix, name);
            restore_tree(hash, &subdir_prefix)?;
        }
    }

    Ok(())
}

fn collect_tree_paths(
    tree_hash: &str,
    path_prefix: &str,
    paths: &mut HashSet<String>,
) -> Result<()> {
    let tree_data = show(&tree_hash.to_string())?;

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

        let mut parts = header.splitn(2, " ");
        let object_type = parts.next().unwrap();
        let name = parts.next().unwrap().trim();

        let full_path = format!("{}{}", path_prefix, name);

        if object_type == "blob" {
            paths.insert(full_path);
        } else if object_type == "tree" {
            paths.insert(full_path.clone() + "/");
            collect_tree_paths(hash, &(full_path + "/"), paths)?;
        }
    }

    Ok(())
}

fn clean_working_dir(
    tree_paths: &HashSet<String>,
    dir: &std::path::Path,
    root: &std::path::Path,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == ".lit" || name == ".git" {
                continue;
            }
        }

        let rel_path = path.strip_prefix(root).unwrap_or(&path);
        let rel_path_str = rel_path.to_str().unwrap();

        if !tree_paths
            .iter()
            .any(|p| rel_path_str == p || rel_path_str.starts_with(p))
        {
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
        } else {
            clean_working_dir(tree_paths, &path, root)?;
        }
    }

    Ok(())
}
