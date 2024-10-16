use std::{cmp::Ordering, io};

use rand::Rng;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess: String = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess:u32 = guess.trim().parse().expect("Please input a number.");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Small!"),
        Ordering::Equal => println!("Equal!"),
        Ordering::Greater => println!("Greater")
    }

    println!("Actual: {}", secret_number);
}
