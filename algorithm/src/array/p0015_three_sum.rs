use std::collections::HashSet;

use crate::Solution;

///  [-3,-2,-1,2,3,5]
///  先排序， a指针从头向后遍历，b指针在a指针之后开始，c指针从结尾开始
///  bc指针指向值和a的值进行累加，如果大于0，则移动c指针，如果小于0，则移动b指针。直到找到或者b==c
///  若找到了bc指针，各减一继续遍历。
///

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort();
        let mut res = vec![];
        for a in 0..nums.len() - 2 {
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }
            if nums[a] > 0 {
                return res;
            }
            //定义指针b
            let mut b = a + 1;
            //定义指针 c
            let mut c = nums.len() - 1;
            //左边的指针小于右边的指针
            while b < c {
                let sum = nums[b] + nums[c] + nums[a];
                if sum < 0 {
                    b += 1;
                } else if sum > 0 {
                    c -= 1;
                } else if sum == 0 {
                    res.push(vec![nums[a], nums[b], nums[c]]);
                    b += 1;
                    c -= 1;
                    while b < c && nums[b] == nums[b - 1] {
                        b += 1;
                    }
                    while b < c && nums[c] == nums[c + 1] {
                        c -= 1;
                    }
                }
            }
        }
        res
    }
    pub fn three_sum_force(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //println!("{:?}",nums);
        let mut set = HashSet::new();
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                for k in j + 1..len {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut s = vec![nums[i], nums[j], nums[k]];
                        s.sort();
                        set.insert(s);
                    }
                }
            }
        }
        //println!("{:?}",set);
        let v = set.into_iter().collect();
        return v;
    }
}

#[test]
fn test() {
    let v = vec![-1, 0, 1, 2, -1, -4];
    let r = Solution::three_sum(v);
    println!("{:?}", r);
}
