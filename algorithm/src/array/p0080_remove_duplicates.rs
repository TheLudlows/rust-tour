use crate::Solution;

impl Solution {
    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut j = 2;
        for i in 2..nums.len(){
            if  nums[i] != nums[j-2]{
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }
}
#[test]
fn test() {
    let mut v = vec![1,1,1,2,2,3];
    Solution::remove_duplicates_2(&mut v);
    println!("{:?}",v);
}