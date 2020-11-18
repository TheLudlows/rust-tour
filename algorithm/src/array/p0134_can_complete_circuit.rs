use crate::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        for i in 0..len {
            let mut j = i;
            let mut remain = gas[i];

            while remain - cost[j] >= 0 {
                remain = remain - cost[j] + gas[(j + 1) % n];
                j = (j + 1) % len;
                if j == i {
                    return i as i32;
                }
            }
        }
        -1
    }
}