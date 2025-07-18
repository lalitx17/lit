use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub fn read_index() -> HashMap<String, String> {
    let mut file_hash: HashMap<String, String> = HashMap::new();
    if let Ok(file) = File::open(".lit/index") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let mut parts = line.split_whitespace();
            if let (Some(hash), Some(file_path)) = (parts.next(), parts.next()) {
                file_hash.insert(file_path.to_string(), hash.to_string());
            }
        }
    }

    return file_hash;
}

pub fn last_commit_hash() -> Result<String> {
    let head_content = fs::read_to_string(".lit/HEAD")?;
    if let Some(ref_path) = head_content.strip_prefix("ref: ").map(str::trim) {
        let ref_file = format!(".lit/{}", ref_path);
        let hash = fs::read_to_string(ref_file)?;
        Ok(hash.trim().to_string())
    } else {
        Ok(head_content.trim().to_string())
    }
}

pub fn is_lit_initialized() -> Result<()> {
    if !std::path::Path::new(".lit").exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "lit is not initialized",
        ))
    } else {
        Ok(())
    }
}
