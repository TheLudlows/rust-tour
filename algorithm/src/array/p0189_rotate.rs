use crate::Solution;

/// 循环替换 O(n)复杂度
impl Solution {
    pub fn rotate_(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        let mut count = 0;
        for start in 0..k {
            let mut index = start;
            let mut prev = nums[index];
            loop {
                index = (index + k) % len;
                let temp = nums[index];
                nums[index] = prev;
                prev = temp;
                count += 1;
                //println!("{:?}",nums);
                if index == start {
                    break;
                }
            }
            if count == len {
                break;
            }
        }
    }
}

#[test]
fn test() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate_(&mut v, 3);
}
