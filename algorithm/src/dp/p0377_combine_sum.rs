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
        find(&nums, target)
    }
}
// 在nums中寻中和为target的数量
fn find(nums: &Vec<i32>, target: i32) -> i32 {
    if target < 0 {
        return 0;
    }
    if target == 0 {
        return 1;
    }
    let mut ret = 0;
    for i in 0..nums.len() {
        ret += find(nums, target - nums[i]);
    }
    ret
}

#[test]
fn test() {
    let v = vec![1,2,3];
    let r = Solution::combination_sum4(v, 20);
    println!("{}", r);
}