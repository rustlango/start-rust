// match is like switch but it is an optimized version of switch

fn main() {
    let marks: i32 = 49;
 
    match marks {
        50..=59 => println!("You passed and got a D symbol"),
    // C symbol else if arm
        60..=69 => println!("You passed and got a C symbol"),
    // anything less than 50 percent is a fail but do not give up:-)
        0..= 49 => println!("Did not pass, however do not gve up - give it another try:-)"),
    // else arm: all other condtions as in great than or equal to 75
        _ => println!("Congratulations: you passed with a distinction"),
    }
}
