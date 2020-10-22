use crate::Solution;

/// 排序
/// Vec last元素的获取最后添加的元素，
/// 逐个和当前的结束值比较，如果当前大于队尾的，那么合并完成，继续下一个合并
/// 否则扩大队尾的范围，或合并进去。
impl Solution {
    pub fn merge_2(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut slice = intervals;
        let mut merge: Vec<Vec<i32>> = Vec::new();
        slice.sort_by(|a, b| a[0].cmp(&b[0]));
        for v in slice {
            let l = v[0];
            let r = v[1];
            if merge.is_empty() || merge.last().unwrap()[1] < l {
                merge.push(v);
            } else {
                merge.last_mut().unwrap()[1] = std::cmp::max(merge.last().unwrap()[1], r);
            }
        }
        merge
    }
}