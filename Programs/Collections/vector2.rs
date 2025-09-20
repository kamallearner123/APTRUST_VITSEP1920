fn main() {
    let v1 = vec![String::from("a"),
                String::from("b"),
                String::from("c")];
    let v2 = vec![5,6,7];

    let a: &String = &v1[0];
    println!("v1  {:?}", v1);



    println!("first eleement = {:?}, index 10 is not there", v1.get(10));


    match v1.get(3) {
        Some(s1) => println!("s1 {}",s1),
        None=>println!("None!!")
    }
}
