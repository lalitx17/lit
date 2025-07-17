use crate::utils::read_index;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::io::Write;

pub fn commit(message: &String) -> Result<()> {
    let file_map = read_index();

    let root_hash = build_tree(&file_map, "")?;

    println!("root_hash: {}", root_hash);

    for (key, value) in &file_map {
        println!("{}: {}", key, value);
    }

    Ok(())
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

fn write_object(object_type: &str, content: &[u8]) -> Result<String> {
    let mut hasher = Sha1::new();
    let header = format!("{} {}\0", object_type, content.len());

    let mut tree_data = Vec::new();
    tree_data.extend_from_slice(header.as_bytes());
    tree_data.extend_from_slice(content);

    hasher.update(&tree_data);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&tree_data)?;
    let compressed_tree_data = encoder.finish()?;

    let (dir, file) = hash_hex.split_at(2);
    let object_dir = format!(".lit/objects/{}", dir);
    let object_path = format!("{}/{}", object_dir, file);

    fs::create_dir_all(&object_dir)?;
    let mut cur_file = fs::File::create(&object_path)?;
    cur_file.write_all(&compressed_tree_data)?;
    Ok(hash_hex)
}

pub fn build_tree(entries: &HashMap<String, String>, prefix: &str) -> Result<String> {
    let mut subdirs: BTreeMap<String, HashMap<String, String>> = BTreeMap::new();
    let mut tree_entries = Vec::new();

    for (path, hash) in entries {
        if let Some(rest) = path.strip_prefix(prefix) {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();

            if parts.len() == 1 {
                let object_type = "blob";
                let name = parts[0];
                let mut entry = Vec::new();
                entry.extend_from_slice(format!("{} {}\0", object_type, name).as_bytes());
                entry.extend_from_slice(hash.as_bytes());
                tree_entries.push(entry);
            } else {
                let dir = parts[0];
                subdirs
                    .entry(dir.to_string())
                    .or_default()
                    .insert(path.clone(), hash.clone());
            }
        }
    }

    for (dir, submap) in subdirs {
        let subtree_hash = build_tree(&submap, &format!("{}/", dir))?;
        let object_type = "tree";
        let mut entry = Vec::new();
        entry.extend_from_slice(format!("{} {}\0", object_type, dir).as_bytes());
        entry.extend_from_slice(subtree_hash.as_bytes());
        tree_entries.push(entry);
    }

    tree_entries.sort_by(|a, b| a.cmp(b));

    let tree_content: Vec<u8> = tree_entries.into_iter().flatten().collect();

    write_object("tree", &tree_content)
}

/* fn build_commit(tree_hash: &String) -> Result<String> {

}
 */
