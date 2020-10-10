use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::{TreeNode, Solution};

/// 层级遍历 队列
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut result = vec![];
        while !queue.is_empty() {
            let n = queue.len();
            for i in 0..n {
                if let Some(Some(node)) = queue.pop_front() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone());
                    }
                    if i == n - 1 {
                        result.push(node.borrow().val);
                    }
                }
            }
        }
        result
    }
}