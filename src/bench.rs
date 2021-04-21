extern crate test;

#[cfg(test)]
mod tests {
    use super::test::{Bencher, bench};
    use super::test::bench::*;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_add_two1(b: &mut Bencher) {
        run_once(|b| add_two(1));
    }
    pub fn add_two(a: i32) {
        println!("{}",1);
        a + 2;
    }
}