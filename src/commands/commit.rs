use crate::utils::read_index;
use std::io::Result;

pub fn commit() -> Result<()> {
    let file_map = read_index();

    for (key, value) in &file_map {
        println!("{}: {}", key, value);
    }

    Ok(())
}
