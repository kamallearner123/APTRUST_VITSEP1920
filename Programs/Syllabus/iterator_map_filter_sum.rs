fn main() {

    let nums = [1,2,3,4];
    let sum1 = nums.iter().map(|x| x*2)
                            .filter(|x| x>5)
                            .sum();
    println!("sum1 = {}", sum1);
}
