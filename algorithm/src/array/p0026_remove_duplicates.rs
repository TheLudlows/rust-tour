use crate::Solution;

/// 双指针
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut j = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[j-1] {
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }

    pub fn remove_duplicates_rs(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[test]
fn test() {
    let mut v = vec![1, 2, 2, 3, 3, 3, 4];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
}