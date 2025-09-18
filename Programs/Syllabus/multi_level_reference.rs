fn main() {
    let mut a:i32 = 10;
    let ra:&mut i32 = &mut a;
    *ra = *ra+20;
    println!("ra={}", *ra);
}
