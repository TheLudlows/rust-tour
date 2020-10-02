use std::collections::HashMap;

use crate::Solution;

/// 遍历数组，将元素加入map，k-v分别是数组元素和下标，插入的时候判断target-x是否在map中，如果在返回结果。
///
/// 排序数组，双指针分别指向首位，然后向中间移动。
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let v = map.get(&(target - *num));
            match v {
                None => { map.insert(num, index); }
                Some(i) => { return vec![*i as i32, (index) as i32]; }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn two_sum_test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}


