use prog39::shapes::{SHAPE, get_area)};


fn main() {
    let r1 = SHAPE::Rectangle(1,2);
    println!("AreA of rectangle = {}", get_area(r1));
    let s1 = SHAPE::Square(2);
    println!("AreA of rectangle = {}", get_area(s1));
}

