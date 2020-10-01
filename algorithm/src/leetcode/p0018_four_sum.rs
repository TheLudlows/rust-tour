use std::fs::read;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if nums.len() < 4 {
            return vec![];
        } else if nums.len() == 4 {
            if nums.iter().sum::<i32>() == target {
                res.push(nums);
                return res;
            }
        }
        nums.sort();
        println!("{:?}", nums);
        for a in 0..nums.len() - 3 {
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }
            for b in a + 1..nums.len() - 2 {
                let mut tag1 = None;
                if b > a + 1 && nums[b] == nums[b - 1] {
                    continue;
                }
                //定义指针c
                let mut c = b + 1;
                //定义指针 d
                let mut d = nums.len() - 1;
                //左边的指针小于右边的指针
                while c < d {
                    let sum = nums[c] + nums[d] + nums[a] + nums[b];
                    if sum < target {
                        c += 1;
                    } else if sum > target {
                        d -= 1;
                    } else if sum == target {
                        if tag1 == Some(nums[c]) { // 去重
                            c += 1;
                            d -= 1;
                            continue;
                        }
                        tag1 = Some(nums[c]);
                        res.push(vec![nums[a], nums[b], nums[c], nums[d]]);
                        c += 1;
                        d -= 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let v = vec![-1, -5, -5, -3, 2, 5, 0, 4];
    let r = Solution::four_sum(v, -7);
    println!("{:?}", r)
}
