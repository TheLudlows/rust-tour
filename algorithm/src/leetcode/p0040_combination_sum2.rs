use std::collections::VecDeque;

/// 回溯 + 减枝
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut result = Vec::new();
        let mut path = Vec::new();
        combination2(&candidates, 0, target, &mut result, &mut path);
        result
    }
}

fn combination2(candidates: &[i32], begin: usize, target: i32, result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
    if target < 0 {// 当前分支不符合
        return;
    }
    if target == 0 {//当前分支符合，加入结果集并返回。
        result.push(Vec::from(path.clone()));
        return;
    }
    if begin == candidates.len() {
        return;
    }
    for i in begin..candidates.len() {
        if i > begin && candidates[i] == candidates[i - 1] { // 去重
            continue;
        }
        // 如果新的target已经小于0，没必要继续当前分支了
        let new_target = target - candidates[i as usize];
        if new_target < 0 {
            return;
        }
        path.push(candidates[i as usize]);
        combination2(candidates, i + 1, new_target, result, path);
        path.pop();
    }
}

#[test]
fn test() {
    let v = vec![2, 3, 6, 7];
    let r = Solution::combination_sum2(v, 7);
    println!("{:?}", r);
}