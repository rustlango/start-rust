// statements do not return a value. Expressions return a value
// notice no need for brackets for the conditional arguments

fn main() {
    let marks: i32 = 75;

    if marks >= 50 && marks <= 59 {
        println!("You passed and got a D symbol");
    }
    else if marks >= 60 && marks < 75 {
        println!("You passed and got a D symbol");
    }

    else if marks < 50 {
        println!("Do not gve up - give it another try:-)");
    }

    else {
        println!("Congratulations: you passed with a distinction");
    }
}
