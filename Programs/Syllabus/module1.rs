pub mod math {
    pub fn add(a:i32, b:i32)->i32{
        a+b
    }
}

fn main() {
    use math::add;
    let r = add(1,2);
    println!("r = {}",r);
}
