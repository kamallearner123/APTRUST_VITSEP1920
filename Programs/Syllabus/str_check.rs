fn just_print(msg : &str) {
    println!("msg = {}", msg);
}



fn main() {
    let a = "Hello";
    just_print(a);
    println!("In main {}", a);
}
