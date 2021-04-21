use crate::Solution;

// 一个数开始count=1，遇到相同的就加1，遇到不同的就减1，减到0就重新换个数开始计数，总能找到最多的那个
// 摩尔投票
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut count, mut most) = (0, nums[0]);

        for element in nums {
            if element == most {
                count += 1;
            }else {
                if count > 0 {
                    count -= 1;
                }else {
                    most = element;
                }
            }
        }
        most
    }
}
