fn main() {
    let mut arr:[i32;6] = [0;6];
    println!("{}", arr[0]);
    

    for i in 0..arr.len() {
        arr[i] = (i as i32)*2;
    }
    for i in 0..arr.len() {
        println!("{}",arr[i]);
    }
}
