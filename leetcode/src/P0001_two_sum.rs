use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (index,num) in nums.iter().enumerate() {
            let v = map.get(&(target - *num));
            match v {
                None => {map.insert(num, index);},
                Some(i) => {return vec![*i as i32, (index) as i32];}
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}


