// immutability vs mutability
// fn main() {
//     let properties = 15;
//     properties += 9;
//     println!("I have a number of properties - they should be {} in total",  properties);
// }
// above code will not compile becuae immutability is a default
// functionality built into rust
// to make variables mutable you need to prepend variable 
// names with the mut keyword

fn main() {
    let mut properties = 15;
    properties += 9;
    println!("I have a number of properties - they should be {} in total",  properties);
}