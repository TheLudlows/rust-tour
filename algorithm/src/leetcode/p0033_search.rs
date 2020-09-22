use crate::leetcode::common::Solution;
///每一次的二分将旋转数组分为了至少有一个有序数组，我们根据有序数组的首尾来判断target是否在其中，如果不在继续二分查找另一半可能无序的数组。
impl Solution {
    fn search_1(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;

        while l <= r {
            let m = (l + r) / 2;
            if target == nums[m] {
                return m as i32;
            }
            if nums[l] <= nums[m] {
                if nums[l] <= target && target <= nums[m] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if nums[m] <= target && target <= nums[r] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }
        -1
    }
}
#[test]
fn test() {
    let v = vec![4,5,6,1,2,3];
    let r = Solution::search_1(v,2);
    println!("{}",r)
}