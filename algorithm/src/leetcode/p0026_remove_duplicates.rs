

/// 双指针
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut last = nums[0];
        let mut i = 1;
        let mut j = 1;
        while i < nums.len() {
            if nums[i] != last {
                nums[j] = nums[i];
                last = nums[i];
                j += 1;
            }
            i += 1;
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