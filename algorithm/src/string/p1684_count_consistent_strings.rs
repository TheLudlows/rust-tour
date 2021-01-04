use crate::Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut set = vec![0; 26];
        let mut ans = 0;

        for byte in allowed.into_bytes() {
            set[(byte - b'a') as usize] = 1;
        }

        for word in words {
            let mut is_allow = true;
            for byte in word.into_bytes() {
                if set[(byte - b'a') as usize] == 0 {
                    is_allow = false;
                    break;
                }
            }
            if is_allow {
                ans += 1;
            }
        }
        ans
    }
}