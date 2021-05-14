//1 <= nums.length <= 200
// 1 <= nums[i] <= 1000
// nums 中的所有元素 互不相同
// 1 <= target <= 1000

//1 <= candidates.length <= 30
// 1 <= candidates[i] <= 200
// candidate 中的每个元素都是独一无二的。
// 1 <= target <= 500

use crate::Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = vec![-1; (target+1) as usize];
        find(&nums, target, &mut memo)
    }

    pub fn combination_sum42(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut memo = vec![0; target+1];
        memo[0] = 1;
        for i in 1..=target {
            for j in 0..nums.len() {
                if i >= nums[j] as usize {
                    memo[i] += memo[i-nums[j] as usize]
                }
            }
        }
        memo[target]
    }
}
// 在nums中寻中和为target的数量
fn find(nums: &Vec<i32>, target: i32, memo: &mut Vec<i32>) -> i32 {
    if target < 0 {
        return 0;
    }
    if target == 0 {
        return 1;
    }
    let idx = target as usize;
    if memo[idx] != -1 {
        return memo[idx];
    }
    let mut ret = 0;
    for i in 0..nums.len() {
        ret += find(nums, target - nums[i], memo);
    }
    memo[idx] = ret;
    ret
}

#[test]
fn test() {
    let v = vec![1,2,3];
    let r = Solution::combination_sum42(v, 20);
    println!("{}", r);
}