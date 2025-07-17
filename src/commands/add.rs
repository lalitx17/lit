use crate::utils::read_index;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, read};
use std::io::{BufRead, BufReader, Result, Write};
use std::path::Path;
use walkdir::WalkDir;

pub fn add(dir_path: &String) -> Result<()> {
    let files = get_all_files_recursively(&dir_path);

    let mut file_map: HashMap<String, String> = read_index();

    for file in &files {
        encrypt_and_store(file, &mut file_map)?;
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

fn create_blob_object(file_path: &str) -> Result<(Vec<u8>, String)> {
    let content = read(file_path)?;
    let size = content.len();

    let header = format!("blob {}\0", size);

    let mut blob_data = Vec::new();
    blob_data.extend_from_slice(header.as_bytes());
    blob_data.extend_from_slice(&content);

    let mut hasher = Sha1::new();
    hasher.update(&blob_data);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob_data)?;
    let compressed_blob = encoder.finish()?;

    Ok((compressed_blob, hash_hex))
}

fn store_hash_to_index(file_map: &HashMap<String, String>) -> Result<()> {
    let mut file = File::create(".lit/index")?;
    for (file_path, hash) in file_map {
        writeln!(file, "{} {}", hash, file_path)?;
    }

    Ok(())
}

pub fn encrypt_and_store(file_path: &String, file_map: &mut HashMap<String, String>) -> Result<()> {
    let (compressed_blob, hash) = create_blob_object(file_path)?;

    if file_map.get(file_path) != Some(&hash) {
        file_map.insert(file_path.clone(), hash.clone());
        store_hash_to_index(&file_map)?;
    }

    let (dir, file) = hash.split_at(2);
    let object_dir = format!(".lit/objects/{}", dir);
    let object_path = format!("{}/{}", object_dir, file);

    fs::create_dir_all(&object_dir)?;

    let mut cur_file = fs::File::create(&object_path)?;
    cur_file.write_all(&compressed_blob)?;
    Ok(())
}
