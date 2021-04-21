use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut map = HashMap::new();
        // 计数
        s.chars().for_each(|c| {
            let temp = map.entry(c).or_insert(0);
            *temp += 1;
        });
        // 放入最大堆 (数量，字符)
        let mut heap = map
            .into_iter()
            .map(|(x, y)| (y, x))
            .collect::<BinaryHeap<_>>();

        // 不能重构的条件
        if heap.len() == 0 || heap.peek().unwrap().0 > (s.len() + 1) / 2 {
            return "".to_string();
        }

        // 取最大的两个元素
        let mut res = String::with_capacity(s.len());
        while heap.len() >= 2 {
            let mut first = heap.pop().unwrap();
            let mut second = heap.pop().unwrap();
            res.push(first.1);
            res.push(second.1);
            first.0 -= 1;
            second.0 -= 1;
            if first.0 > 0 {
                heap.push(first);
            }
            if second.0 > 0 {
                heap.push(second);
            }
        }
        // 只剩一个字符了, 只能有一次了
        if heap.len() == 1 {
            let temp = heap.pop().unwrap();
            res.push(temp.1);
        }

        res
    }
}