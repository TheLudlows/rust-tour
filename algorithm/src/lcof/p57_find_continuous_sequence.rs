use crate::Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut sum;
        let limit  = (target+1)/2;
        let mut result = vec![];
        for i  in 1..limit {
            sum = 0;
            for j  in i..limit {
                sum +=j;
                if sum == target {
                    let mut r = vec![];
                    for k in i..=j {
                        r.push(k);
                    }
                    result.push(r);
                    break;
                } else if sum > target {
                    break;
                }
            }
        }
        result
    }
}
#[test]
fn test() {
    Solution::find_continuous_sequence(9);
}