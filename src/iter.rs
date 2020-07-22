fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1).count();

    let v1: Vec<i32> = vec![1, 2, 3];
    let i = 2;
    let sum:i32 = v1.iter().filter(|e| e > &&i).sum();
    assert_eq!(sum,3);


    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
}