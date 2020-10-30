use crate::Solution;

impl Solution {
    pub fn three_sum_close(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut close = nums[0] + nums[1] + nums[2];
        for a in 0..nums.len() - 2 {
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }
            //定义指针b
            let mut b = a + 1;
            //定义指针 c
            let mut c = nums.len() - 1;
            //左边的指针小于右边的指针
            while b < c {
                let sum = nums[b] + nums[c] + nums[a];
                if (sum - target).abs() < (close - target).abs() {
                    close = sum;
                }
                if sum < target {
                    b += 1;
                } else if sum > target {
                    c -= 1;
                } else {
                    return close;
                }
            }
        }
        close
    }
}
