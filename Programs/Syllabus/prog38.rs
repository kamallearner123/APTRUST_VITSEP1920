enum DAY {
    SUNDAY,
    MONDAY,
    TUSEDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY
}

fn check_week_end(d:DAY) -> bool {
    match d {
        DAY::SUNDAY|DAY::SATURDAY => {println!("No..."); false},
        _ => true
    }
}
fn main() {
    println!("Is SUNDAY week end {}", check_week_end(DAY::MONDAY));
}
