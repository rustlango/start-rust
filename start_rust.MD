HOW TO SETUP RUST PROJECT USING rust's package manger called cargo

We're going to be making a binary project; the other option is a library.

cargo new will create a skeleton project setup for you.

cd to a directory where you would like to store code
cargo new start_rust
cd start rust

____________________

GENERATED FILES WITH cargo new COMMAND

Cargo.toml: provides metadata related to your project and the packages/dependencies used
.gitignore: ignores the compiled files built by Rust when adding them to Git
src/main.rs: All your rust code should go in the sub-directory of your project called src/


_____________________

EXAMPLE OF TOML FILE

[package]
name = "start_rust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

____________________

src/ DIRECTORY

cargo new generates a main.rs file containing a hello world program which
you can quickly check to see if it compiles and executes using one single commnd cargo run

src/main.rs
fn main() {
    println!("Hello, world!");
}


____________________

cargo run REVEALED

Running cargo run command should print "Hello, world!"

cargo run generates extra files as described below:
target directory: all the built stuff (won't be added to Git)
Cargo.lock: locks your dependencies (which we do not have as yet)

____________________

SYNTAX AND CONCEPTS

COMMENTS

typing a double shash,  // , at the start of a line makes a comment 

This is the common manner in which you comment in rust though there are other ways


____________________

TYPES IN RUST

Rust is a strongly statically typed programming language. A list of few of the main types are as follows:

u32: unsigned 32-bit integer
i32: signed 32-bit integer
f64: floating point number
String and/or &str: will explain the difference
bool: a boolean

TYPE INFERENCE RUST

Every value has a type that the compiler has to know about.
Most of the time, the compiler can infer.
Sometimes it can't, and you'll get an error and need to add an annotation.

Rust's compiler can makes type inferences but it is best to annotate the desired type upon varaible decalration

Instead of let properties = 8; one can write let properties: u32 = 30;
Function definitions should always include type annotations for the parameters and the return evaluation.

____________________

CONDITIONALS

if:

if else if else - these statements do not need brackets

match:

mathc is a better version of switch