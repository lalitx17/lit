use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use walkdir::WalkDir;

pub fn add(dir_path: &String) -> Result<()> {
    let mut files: Vec<String> = Vec::<String>::new();
    files = get_all_files_recursively(&dir_path);

    for file in &files {
        println!("{}", file);
    }

    Ok(())
}

fn is_ignored(path: &String, ignore_files: &[String]) -> bool {
    let path_obj = Path::new(path);
    for file in ignore_files {
        if file.is_empty() || file.starts_with('#') {
            continue;
        }

        if path_obj
            .components()
            .any(|comp| comp.as_os_str() == std::ffi::OsStr::new(file))
        {
            return true;
        }
    }

    return false;
}

pub fn get_all_files_recursively(dir_path: &String) -> Vec<String> {
    let mut ignore_files = Vec::new();
    if let Ok(file) = File::open(".litignore") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            ignore_files.push(line.trim().to_string());
        }
    }

    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| {
            let path_str = e.path().display().to_string();
            path_str.strip_prefix("./").unwrap_or(&path_str).to_string()
        })
        .filter(|path| !is_ignored(&path, &ignore_files))
        .collect()
}
