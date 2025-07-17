use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{Read, Result};

pub fn show(hash: &String) -> Result<String> {
    let (dir, file) = hash.split_at(2);
    let object_path = format!(".lit/objects/{}/{}", dir, file);

    let f = File::open(&object_path)?;
    let mut decoder = ZlibDecoder::new(f);
    let mut contents = Vec::new();
    decoder.read_to_end(&mut contents)?;

    if let Some(pos) = contents.iter().position(|&b| b == 0) {
        let body = &contents[pos + 1..];
        Ok(String::from_utf8_lossy(body).to_string())
    } else {
        Ok(String::from_utf8_lossy(&contents).to_string())
    }
}
