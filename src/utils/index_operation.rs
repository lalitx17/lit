use std::collections::HashMap;
use std::fs::File;
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
