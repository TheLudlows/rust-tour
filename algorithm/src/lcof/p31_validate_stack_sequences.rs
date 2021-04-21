use crate::Solution;

///初始化栈 stack，j = 0；
/// 遍历 pushed 中的元素 x；
/// 当 j < popped.size() 且栈顶元素等于 popped[j]：
/// 弹出栈顶元素；
/// j += 1；
/// 如果栈为空，返回 True，否则返回 False。

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut j = 0;
        for i in pushed {
            stack.push(i);
            while !stack.is_empty() && popped[j] == *stack.last().unwrap() {
                stack.pop();
                j += 1;
            }
        }
        stack.is_empty()
    }
}