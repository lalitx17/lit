use std::fs;
use std::io::Result;
use std::io::Write;

pub fn init() -> Result<()> {
    fs::create_dir_all("./lit/objects")?;
    fs::create_dir_all("./lit/refs/heads")?;
    fs::write(".lit/HEAD", "ref: refs/heads/master\n")?;
    fs::File::create(".lit/refs/heads/main")?.write_all(b"")?;
    fs::File::create(".lit/index")?;
    println!("Initialized empty lit repository in .lit");

    Ok(())
}
