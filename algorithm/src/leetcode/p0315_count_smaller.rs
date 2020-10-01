

/// 线段树
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        let n = nums.len();
        for i in 0..n {
            let mut x = 0;
            for j in i..n {
                if nums[i] > nums[j] {
                    x += 1;
                }
            }
            result.push(x);
        }
        result
    }
}