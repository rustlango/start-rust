// statements do not return a value. Expressions return a value
// notice no need for brackets for the conditional arguments

fn main() {
    let marks: i32 = 75;
    // D symbol if arm
    if marks >= 50 && marks <= 59 {
        println!("You passed and got a D symbol");
    }
    // C symbol else if arm
    else if marks >= 60 && marks < 75 {
        println!("You passed and got a C symbol");
    }
    // anything less than 50 percent is a fail but do not give up:-)
    else if marks < 50 {
        println!("Do not gve up - give it another try:-)");
    }
    // else arm: all other condtions as in great than or equal to 75
    else {
        println!("Congratulations: you passed with a distinction");
    }
}
