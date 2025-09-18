use std::io;

fn main() {
    let i1 = io::stdin();
    let mut data = String::new();
    println!("i1 = {:?}",i1.read_line(&mut data).expect("sorry"));
    println!("line = {}",data);
}
