use crate::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ret = vec![];

        for i in 1..=n {
            if i%15 == 0 {
                ret.push("FizzBuzz".to_string());
            } else if i%5 == 0 {
                ret.push("Buzz".to_string());
            } else if i%3 == 0{
                ret.push("Fizz".to_string());
            } else {
                ret.push(i.to_string());
            }
        }
        ret
    }
}