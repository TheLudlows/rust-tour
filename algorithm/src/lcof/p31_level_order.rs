use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut result = vec![];
        while !queue.is_empty() {
            let n = queue.len();
            if let Some(Some(node)) = queue.pop_front() {
                result.push(node.borrow().val);
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
        }
        result
    }
}