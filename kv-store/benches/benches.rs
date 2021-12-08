use criterion::Criterion;
use std::time::Duration;


mod arena;
mod fib;
fn main() {
    let mut c = Criterion::default()
        // Configure defaults before overriding with args.
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1))
        .configure_from_args();
    //arena::bench_vec(&mut c);
    //arena::bench_arena(&mut c);
    fib::fibonacci_benchmark(&mut c);
    c.final_summary();
}
