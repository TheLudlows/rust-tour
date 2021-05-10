use crate::Solution;

// 双指针, 遇到相同的字母，选择消耗小的，剩下的继续和后面的比较，i j不同时，j向后移动，i指向当前j
impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let bytes = s.as_bytes();
        let mut c = 0;
        let mut i = 0;
        for j in 1..bytes.len() {
            if bytes[i] == bytes[j] {
                c += std::cmp::min(cost[i], cost[j]);
                if cost[j] > cost[i] { i = j };
            } else {
                i = j;
            }
        }
        c
    }
}
//" a a a    b b b a b b b b"
// [-3,5,-10,7,-5,-3,-5,-5,-4,8,-1]