use crate::Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(&mut nums,0,nums.len()-1);
        nums
    }
    pub fn quick_sort(nums: &mut Vec<i32>,l:usize,r:usize) {
        if nums.len() <= 1 {
            return;
        }
        let
        for i in l..r-1 {
            if nums[i] >= nums[0] {
                vec2.push(nums[i]);
            } else {
                vec1.push(nums[i]);
            }
        }
        let mut result = Self::quick_sort(vec1);
        result.push(nums[0]);
        result.append(&mut Self::quick_sort(vec2));

        //println!("{:?}",result);
        result
    }
}