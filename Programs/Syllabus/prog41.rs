trait animal {
    fn make_sound(&self) {
        println!("....");
    }
}


struct Dog{
    name:String
}
struct Cat;

impl animal for Dog {
    fn make_sound(&self) {
        println!("baw..baw...");
    }
}

impl animal for Cat {
    fn make_sound(&self) {
        println!("meaw..meaw...");
    }
}
    

fn main() {
    let d1 = Dog{name:String::from("darbarman")};
    let c1 = Cat;

    d1.make_sound();
    c1.make_sound();
}

