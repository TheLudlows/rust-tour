use crate::Solution;

/// 双指针
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 1 {
            return 0
        }
        let (mut i, mut j) = (0, 1);
        while j < nums.len() {
            if nums[i] != nums[j] {
                i+=1;
                nums[i] = nums[j];
            }
            j+=1;
        }
        i as i32
    }

    pub fn remove_duplicates_rs(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[test]
fn test() {
    let mut v = vec![1, 1, 2];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
}
