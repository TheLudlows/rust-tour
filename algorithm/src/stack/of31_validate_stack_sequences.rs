use crate::Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut idx = 0;
        for elem in pushed {
            stack.push(elem);
            while !stack.is_empty() && popped[idx] == *stack.last().unwrap() {
                stack.pop();
                idx += 1;
            }
        }
        stack.is_empty()
    }
}