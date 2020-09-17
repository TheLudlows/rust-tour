use crate::leetcode::common::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //为了对付 leetcode 给的 比如像 [] [0] [1,2] 这样的没用数据
        if nums.len() < 3 { return vec![];}
        let mut nums = nums;
        //作下 排序
        nums.sort();
        let mut res =vec![];
        //定义 c
        for pointer_c in 0..nums.len() -2{
            let mut tag1 = None;
            if pointer_c >0 && nums[pointer_c] == nums[pointer_c -1] { continue; }
            //定义指针a 要比 c 大
            let mut pointer_a= pointer_c + 1;
            //定义指针 b
            let mut pointer_b =nums.len() -1;
            //左边的指针小于右边的指针
            while(pointer_a < pointer_b ){
                //如果 a + b < -c 那么 右移动a 指针 a + b 变大
                if nums[pointer_a] +  nums[pointer_b] < -nums[pointer_c] {
                    pointer_a += 1;
                    //如果 a + b > -c 那么 左移动b 指针 a + b 变小
                }else if nums[pointer_a] +  nums[pointer_b] >  -nums[pointer_c]{
                    pointer_b -= 1;
                    //如果 a + b = -c 找到了
                }else if nums[pointer_a] +  nums[pointer_b] ==  -nums[pointer_c]{
                    if tag1 == Some(nums[pointer_a])  { pointer_a += 1;pointer_b -= 1; continue;}
                    tag1 = Some(nums[pointer_a]);
                    res.push(vec![nums[pointer_c],nums[pointer_a],nums[pointer_b]]);
                    pointer_a += 1;
                    pointer_b -= 1;

                }
            }


        }
        res

    }
}