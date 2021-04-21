use crate::Solution;

/// 双指针
impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let bytes = s.as_bytes();
        let mut c = 0;
        let mut j = 0;
        for i in 1..bytes.len() {
            if bytes[i] == bytes[j] {
                c += std::cmp::min(cost[i], cost[j]);
                j = if cost[j] > cost[i] { j } else { i };
            } else {
                j = i;
            }
        }
        c
    }
}
//" a a a    b b b a b b b b"
// [-3,5,-10,7,-5,-3,-5,-5,-4,8,-1]