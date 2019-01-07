// get the input/output library from the standard library
use std::io;

// Get guess from user
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // create mutable variable bound to a new, empty instance of a String
    let mut guess = String::new();

    // call 'assoc. fxn' on io, get input, and make it a mutable 'reference'
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

// Generating a Secret Number
