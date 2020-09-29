use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::common::{Solution, TreeNode};

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) =>
                1 + Solution::count_nodes(node.borrow().left.clone()) +
                    Solution::count_nodes(node.as_ref().borrow().right.clone()),
            None => 0
        }
    }
}

#[test]
pub fn test() {}