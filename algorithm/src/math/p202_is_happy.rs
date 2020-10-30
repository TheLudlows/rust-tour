use std::collections::HashSet;

use crate::Solution;
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut set = HashSet::new();
        set.insert(n);
        loop {
            let mut cur = 0;
            while n > 0 {
                cur += (n % 10).pow(2);
                n = n / 10;
            }
            if cur == 1 {
                return true;
            }
            if set.contains(&cur) {
                return false;
            } else {
                set.insert(cur);
            }
            n = cur;
        }
    }
}
