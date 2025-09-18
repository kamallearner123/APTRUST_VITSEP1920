use std::fs::File;
use std::io::Write;


fn main() -> std::io::Result<()>{

    let mut file = File::create("book2.txt")?;
    file.write_all(b"All the best123!!!")?;
    Ok(())
}
