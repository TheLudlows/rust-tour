use crate::Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut x, mut n) = if n < 0 {
            (1f64 / x, -n)
        } else {
            (x, n)
        };

        let mut pow = 1f64;

        if n == 0 { return 1f64; }

        while n != 1 {
            if n % 2 == 1 {
                pow = pow * x;
            }
            n = n / 2;
            x = x * x;
            if x == 0f64 || x == 1f64 {
                break;
            }
        }
        pow * x
    }
}

#[test]
fn test() {
    let r = Solution::my_pow(2f64, 4);
    println!("{}", r);
}
