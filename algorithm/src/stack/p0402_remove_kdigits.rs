use crate::Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut res = String::new();
        for n in num.chars() {
            while k > 0 && !res.is_empty() && n < res.chars().last().unwrap() {
                k -= 1;
                res.pop();
            }
            // removing leading zeros
            if !res.is_empty() || n != '0' {
                res.push(n);
            }
        }

        // make sure remove k digits in total
        while !res.is_empty() && k > 0 {
            res.pop();
            k -= 1;
        }

        return if !res.is_empty() {
            res
        } else {
            "0".to_string()
        }
    }
}

#[test]
fn test() {
    let r = Solution::remove_kdigits("012345".to_string(),3);
    println!("{}",r)
}