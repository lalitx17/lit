use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{Read, Result};

pub enum ShowResult {
    Exists(String),
    NotFound,
}

pub fn show(hash: &String) -> Result<ShowResult> {
    let (dir, file) = hash.split_at(2);
    let object_path = format!(".lit/objects/{}/{}", dir, file);

    let f = match File::open(&object_path) {
        Ok(file) => file,
        Err(_) => {
            return Ok(ShowResult::NotFound);
        }
    };
    let mut decoder = ZlibDecoder::new(f);
    let mut contents = Vec::new();
    decoder.read_to_end(&mut contents)?;

    let result = if let Some(pos) = contents.iter().position(|&b| b == 0) {
        let body = &contents[pos + 1..];
        String::from_utf8_lossy(body).to_string()
    } else {
        String::from_utf8_lossy(&contents).to_string()
    };
    Ok(ShowResult::Exists(result))
}
