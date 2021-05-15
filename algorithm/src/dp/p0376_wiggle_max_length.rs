use crate::Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut memo = vec![vec![-1; nums.len()]; 2];
        for i in 0..nums.len() {
            ret = std::cmp::max(ret, find(&nums, i, 0, &mut memo));
            ret = std::cmp::max(ret, find(&nums, i, 1, &mut memo));
        }
        ret
    }
    // 二维DP
    pub fn wiggle_max_length2(nums: Vec<i32>) -> i32 {}
}

// 以idx结尾的摇摆序列的最大长度
fn find(nums: &Vec<i32>, idx: usize, flag: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    let mut ret = 1;
    if memo[flag][idx] != -1 {
        return memo[flag][idx];
    }
    for i in 0..idx {
        if flag == 1 {
            if nums[i] > nums[idx] {
                ret = std::cmp::max(ret, 1 + find(nums, i, 0, memo));
            }
        } else {
            if nums[i] < nums[idx] {
                ret = std::cmp::max(ret, 1 + find(nums, i, 1, memo));
            }
        }
    }
    memo[flag][idx] = ret;
    ret
}

#[test]
fn test() {
    let v = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    let r = Solution::wiggle_max_length(v);
    println!("{}", r);
}