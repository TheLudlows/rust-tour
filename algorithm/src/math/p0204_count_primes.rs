use crate::Solution;

//  暴力法
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut count = 0;
        for i in 2..n {
            if isPrime(i) {
                count += 1;
            }
        }
        count
    }
}

fn isPrime(n: i32) -> bool {
    let x = (n as f32).sqrt() as i32;
    for i in 2..=x {
        if n % i == 0 {
            return false
        }
    }
    true
}

#[test]
fn test() {
    println!("{}", Solution::count_primes(10))
}

// 埃式筛
pub fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    let mut d = vec![true; n];
    let mut count = 0;
    for i in 2..n {
        if d[i] {
            count += 1;
            let mut j = i*i;
            while j < n {
                d[j] = false;
                j += i;
            }
        }
    }
    count
}
