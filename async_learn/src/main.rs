#![feature(generator_trait)]
#![feature(generators)]
use std::ops::Generator;
fn main() {
    let mut gen = move || {
        yield 1;
        yield 2;
        return 3;
    };

    unsafe {
        for _ in 0..3 {
            let s = gen.resume();
            println!("{:?}", s);
        }
    }
}