
fn access(v1:&mut Vec<i32>) {
    v1.push(100);
}


fn main() {
    let mut a:Vec<i32> = vec![1,2,3];
    access(&mut a); //b=a
    println!("a = {:?}",a);
}
