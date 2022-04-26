// match is like switch but it is an optimized version of switch

// fn main() {
//     let marks: i32 = 89;
 
//     match marks {
//         // anything less than 50 percent is a fail but do not give up:-)
//         0..= 49 => println!("Did not pass, however do not gve up - give it another try:-)"),
//         // D symbol match
//         50..=59 => println!("You passed and got a D symbol"),
//         // C symbol match
//         60..=69 => println!("You passed and got a C symbol"),
//         // else arm: all other condtions as in great than or equal to 75
//         _ => println!("Congratulations: you passed with a distinction"),
//     }
// }
use std::io;

fn main() {
    println!("Please enter your grade score:");
    
    let mut marks = String::new();

    io::stdin()
        .read_line(&mut marks)
        .expect("Failed to read line");

        let marks: usize = marks
            .trim()
            .parse()
            .expect("Mark or grade entered was not a numerical value");

    match marks {
        // anything less than 50 percent is a fail but do not give up:-)
        0..= 49 => println!("Did not pass, however do not gve up - give it another try:-)"),
        // D symbol match
        50..=59 => println!("You passed and got a D symbol"),
        // C symbol match
        60..=69 => println!("You passed and got a C symbol"),
        // else arm: all other condtions as in great than or equal to 75
        _ => println!("Congratulations: you passed with a distinction"),
    }
}