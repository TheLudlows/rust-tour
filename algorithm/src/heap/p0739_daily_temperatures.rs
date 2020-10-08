use crate::Solution;
/// mpl Solution {
    pub fn daily_temperatures( t: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans =vec![0,t.len()];
        for i in 0..t.len() {
            while !stack.is_empty() && t[i] > t[stack.last().unwrap()] {
                let j = stack.pop().unwrap();
                ans[j] = (i - j) as i32;
            }
            stack.push(i);
        }
        t
    }
}