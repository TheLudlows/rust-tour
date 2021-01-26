use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dep(root.as_ref())
    }
}

fn dep(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(n) = node {
        1 + max(
            dep(n.borrow().left.as_ref()),
            dep(n.borrow().right.as_ref()),
        )
    } else {
        0
    }
}
