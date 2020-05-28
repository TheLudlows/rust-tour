use std::{io, env};
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
pub mod mods;
pub mod common;
pub mod enums;
pub mod closures;
pub mod iter;
pub mod boxes;
pub mod var;
pub mod rc;
pub mod refcell;
pub mod rcrefcell;
pub mod weakref;
pub mod weak2;
pub mod weak3;

use common::utils as u;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    guess.clear();
    common::utils::print(&String::from("aa"));

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

