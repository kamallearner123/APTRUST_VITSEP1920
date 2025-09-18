use std::fs::File;
use std::io::Read;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct book {
    name:String,
    date:String,
    quantity:i32
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("src/book2.json")?;
    
    let mut data:String = String::new();

    file.read_to_string(&mut data)?;

    let record:book = serde_json::from_str(&data).expect("JSON parse error");
    println!("Record = {:?}", record);

    Ok(())
}
