use criterion::Criterion;

// criterion_demo/src/lib.rs
pub fn slow_fibonacci(nth: usize) -> u64 {
    if nth <= 1 {
        return nth as u64;
    } else {
        return slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2);
    }
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..nth {
        b = a + b;
        a = b - a;
    }
    b
}
pub fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| b.iter(|| slow_fibonacci(20)))
        .bench_function("fib", |b| b.iter(|| fast_fibonacci(20)));
}
