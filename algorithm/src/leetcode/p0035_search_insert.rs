use crate::leetcode::common::Solution;
///
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i <= j {
            let mid = (i + j) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                i=mid+1;
            }else {
                if mid == 0 {
                    break
                }
                j=mid-1;
            }
        }
        i as i32
    }
}

#[test]
fn test() {
    let v = vec![1,2,3,4,5];
    let r = Solution::search_insert(v,0);
    println!("{}",r)
}