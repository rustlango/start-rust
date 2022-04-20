// fn main() {
//     let properties = 5;
//     properties += 3;
//     println!("I have a number of properties - they should be {} in total",  properties);
// }

// the above code will not compile. Because in rust variables are immutable by default
// the correct way is to prepend the mut keyword to the variable name

fn main() {
    let mut properties = 15;
    properties += 9;
    println!("I have a number of properties - they should be {} in total",  properties);
}