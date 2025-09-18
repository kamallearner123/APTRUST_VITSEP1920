fn fun1(s1:&String) {
    println!("s1 = {}",s1);
}

fn main() {
    let s1=String::from("hello");
    fun1(&s1);
}
