trait greet {
    fn hello(&self);
}

struct Person {
    name:String
}

impl greet for Person {
    fn hello(&self) {
        println!("Hello {}", self.name);
    }
}

fn check<T:greet>(data:T) {
    data.hello();
}

fn main() {
    let p1 = Person{name:"Kamal".to_string()};
    p1.hello();
    check(p1);
}
