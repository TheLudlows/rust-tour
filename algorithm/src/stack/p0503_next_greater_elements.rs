use crate::Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums.len()];
        let mut stack = vec![];

        for i in 0..(2*nums.len()-1) {
            let idx = i%nums.len();
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[idx]{
                let little_idx = stack.pop().unwrap();
                ret[little_idx] = nums[idx];
            }
            stack.push(idx)
        }
        ret
    }
}
#[test]
fn test() {
    Solution::next_greater_elements(vec![1,2,1]);
}
