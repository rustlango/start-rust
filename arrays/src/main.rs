// fixed length compound data type of one type. Cannot use different 
// value tyupes in a rust array
// square bracket notation wih values separated by commas
// suitable for fixed elements like days of the week
// use vector for unfixed elements of data


// a few ways to declare or create an array

// 1. Array declared without datat type and size
fn main() {
    let an_array = [1, 2, 3];
    // {} will not work when printing arrays to the screen so use {}:?} fmt
    println!("An array declared without data type and size{:?} ", an_array);
    // checking the lenght of an array using the standard len() function
    println!("This array contains {} elements", an_array.len());

// 2. Declaring an array by sepcifying type and size
// array: [T; N] T is the type and N is the number of elements

    let an_array1: [i32; 3] = [1, 2, 3];
    // {} will not work when printing arrays to the screen so use {}:?} fmt
    println!("
    ");
    println!("An array declared with data type and size{:?} ", an_array1);
    // checking the lenght of an array using the standard len() function
    }
