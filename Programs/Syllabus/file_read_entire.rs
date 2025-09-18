use std::fs::File;
use std::io;
use std::io::{BufRead};


fn main() -> std::io::Result<()> {

    let file = File::open("file_read_entire.rs")?;
    
    let reader = io::BufReader::new(file);

    for i in reader.lines() {
        println!(" i = {}", i?);
    }

    Ok(())

}
