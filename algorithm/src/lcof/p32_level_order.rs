use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut result = vec![];
        let mut i = 0;
        while !queue.is_empty() {
            let n = queue.len();
            let mut level = vec![];
            for _ in 0..n {
                if let Some(Some(node)) = queue.pop_front() {
                    level.push(node.borrow().val);
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                }
            }
            if level.len() != 0 {
                if i & 1 == 1 {
                    level.reverse();
                }
                result.push(level);
            }
            i += 1;
        }
        result
    }
}