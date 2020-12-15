extern crate test;

#[cfg(test)]
mod tests {
    use super::test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
}