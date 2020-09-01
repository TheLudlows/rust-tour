#![feature(generator_trait)]
#![feature(generators)]

use std::ops::Generator;
use std::pin::Pin;

fn main() {
    let mut gen = move || {
        yield 1;
        yield 2;
        return 3;
    };
    for _ in 0..3 {
        let s = Pin::new((&mut gen)).resume(());
        println!("{:?}", s);
    }
}