use std::collections::HashMap;

use crate::Solution;
// 可能有负数， 不能用滑动窗口
impl Solution {
    /// 暴力
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    count += 1;
                }
            }
        }
        count as i32
    }

    /// 保存前缀和的数量
    pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();
        let mut cur = 0;
        map.insert(0, 1);
        for i in nums {
            cur += i;
            count += map.get(&(cur - k)).unwrap_or(&0);
            let n = map.entry(cur).or_insert(0);
            *n += 1;
        }
        count as i32
    }
}

#[test]
fn test() {}
