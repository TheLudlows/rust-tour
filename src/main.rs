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
pub mod thread1;
pub mod thread2;
pub mod thread3;
pub mod thread5;
pub mod thread6;

pub mod moshi;

pub mod unsafes;
pub mod trait2;
pub mod trait3;
pub mod trait4;
pub mod trait6;
pub mod method2;
pub mod FlowControl;
pub mod array;
pub mod range;
pub mod slice2;
pub mod str;
pub mod rowp;
pub mod never;
pub mod tuple;
pub mod structs;
pub mod dequeue;
pub mod map;
pub mod heap;
pub mod trait7;
pub mod trait8;
pub mod trait9;
pub mod trait10;
pub mod clone;
pub mod sendsync;
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

