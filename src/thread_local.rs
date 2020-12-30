use std::time::Instant;
use std::cell::Cell;
thread_local! {
  static COUNTER: Cell<u32> = Cell::new(0);
}

const STEPS: u32 = 1_000_000_000;
fn sum_rust() -> u32 {
    for step in 0..STEPS {
        COUNTER.with(|it| {
            let inc = step.wrapping_mul(step) ^ step;
            it.set(it.get() + 1)
        })
    }
    COUNTER.with(|it| it.get())
}
fn main() {
    let t = Instant::now();
    let r = sum_rust();
    eprintln!("Rust:   {} {}ms", r, t.elapsed().as_millis());
}