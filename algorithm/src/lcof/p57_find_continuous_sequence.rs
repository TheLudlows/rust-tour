use crate::Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut sum;
        let limit = (target + 1) / 2;
        let mut result = vec![];
        for i in 1..limit {
            sum = 0;
            for j in i..limit {
                sum += j;
                if sum == target {
                    result.push((i..=j).into_iter().collect());
                    break;
                } else if sum > target {
                    break;
                }
            }
        }
        result
    }

    pub fn find_continuous_sequence2(target: i32) -> Vec<Vec<i32>> {
        let mut sum = 3;
        let mut i = 1;
        let mut j = 2;
        let mut result = vec![];
        while i < j {
            if sum == target {
                result.push((i..=j).into_iter().collect());
            }
            if sum >= target {
                sum -= i;
                i += 1;
            } else {
                j += 1;
                sum += j;
            }
        }
        result
    }
}

#[test]
fn test() {
    Solution::find_continuous_sequence2(9);
}