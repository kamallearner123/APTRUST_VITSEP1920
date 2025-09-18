fn longest<'a> (x:&'a String,
    y:&'a String) -> &'a String {
    
    if x.len()>y.len() {
        return x;
    }
    else {
        return y;
    }
}


fn main() {
    let a1 = String::from("hello");
    let a2 = String::from(" world");
    let r = longest(&a1, &a2);
    println!("r = {}", r)
}

