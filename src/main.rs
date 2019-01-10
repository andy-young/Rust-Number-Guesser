// get external dependency
extern crate rand;

// get the input/output library from the standard library
use std::io;

//
use std::cmp::Ordering;

// Rng trait defines methods that random num gens use
use rand::Rng;

// Get guess from user
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // create mutable variable bound to a new, empty instance of a String
    let mut guess = String::new();

    // call 'assoc. fxn' on io, get input, and make it a mutable 'reference'
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
