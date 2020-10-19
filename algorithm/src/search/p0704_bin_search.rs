use crate::Solution;

impl Solution {
    fn search_left(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    }

    fn search_right(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] > target {
                right = mid;
            } else {
                left = mid+1;
            }
        }
        if nums[left-1] == target {
            (left-1) as i32
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let v = vec![1, 1, 2, 2, 3, 3, 4];
    let r = Solution::search_right(v, 2);
    println!("{}", r)
}