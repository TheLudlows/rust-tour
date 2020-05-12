use std::io;
pub mod func ;
pub mod vector;
pub mod string;
pub mod hashmap;
pub mod error;
pub mod error2;
pub mod error3;
pub mod generic;
pub mod traits;
pub mod lifecircle;


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    guess.clear()

}

