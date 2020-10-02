use std::fs::read;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let s = str.trim();
        let mut n: i64 = 0;
        let mut negative = false;

        let MIN: i64 = -2_147_483_648 as i64;
        let MAX: i64 = 2_147_483_647 as i64;
        for (i, c) in s.chars().enumerate() {
            if c == '-' {
                negative = true;
                if i != 0 {
                    return 0;
                }
                continue;
            } else if c == '+' {
                if i != 0 {
                    return 0;
                }
                continue;
            } else if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as i64;
                if -n < MIN && negative {
                    return MIN as i32;
                }
                if n > MAX && !negative {
                    return MAX as i32;
                }
            } else {
                break;
            }
        }
        if negative {
            if -n < MIN {
                return MIN as i32;
            } else {
                return -n as i32;
            }
        } else {
            if n > MAX {
                return MAX as i32;
            } else {
                return n as i32;
            }
        }
    }
}

struct Solution;

#[test]
fn test() {
    let n = Solution::my_atoi("-2147483648".to_string());
    assert_eq!(n, i32::MIN)
}
