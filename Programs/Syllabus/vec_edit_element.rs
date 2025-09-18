fn main() {
    let a1 = [1,2,3];
    let mut v1:Vec<String> = Vec::new();
    for i in a1 {
        println!("{}",i);
        v1.push(i.to_string());
    }
    println!("v1 = {:?}", v1);
    let mut a:&mut String = &mut v1[0]; // Taking out first element
    a.push_str("___");
    println!("v1 = {:?}", v1);

}
