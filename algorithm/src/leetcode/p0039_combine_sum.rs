use crate::leetcode::common::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        combination(&candidates, 0, target)
    }
}

fn combination(candidates: &[i32], prev: i32, target: i32) -> Vec<Vec<i32>> {
    // if find a solution, return `vec![vec![]]`
    if target == 0 {
        vec![vec![]]
    } else {
        let mut solutions = vec![];
        for candidate in candidates {
            // because candidates in ascending order,
            // no need to check the rest of candidates.
            if target < *candidate {
                break;
            }

            // keep all solutions in ascending order.
            if prev > *candidate {
                continue;
            }

            // find the solution recursively.
            let rest = combination(candidates, *candidate, target - candidate);

            // construct the solution
            for mut each in rest {
                each.push(*candidate);
                solutions.push(each);
            }
        }
        solutions
    }
}