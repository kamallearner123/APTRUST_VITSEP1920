pub mod shapes {

pub enum SHAPE {
    Square(i32),
    Rectangle(i32,i32)
}

pub fn get_area(data:SHAPE) ->i32 {
    match data {
        SHAPE::Square(a) => a*a,
        SHAPE::Rectangle(a,b) => a*b
    }
}

