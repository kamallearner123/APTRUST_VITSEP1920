fn main() {
    let mut data = String::from("Hello");
    let rdata1 = &data;
    let rdata2 = &data;
    let rdata3 = &data;

    data.push('.');
    
//    println!("rdata1 = {}", rdata1);
}
