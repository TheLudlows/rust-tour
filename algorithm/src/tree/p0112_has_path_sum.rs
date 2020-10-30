use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        path_sum(root.as_ref(), sum)
    }
}

fn path_sum(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    match root {
        Some(node) => {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                sum == node.borrow().val
            } else {
                path_sum(node.borrow().left.as_ref(), sum - node.borrow().val)
                    || path_sum(node.borrow().right.as_ref(), sum - node.borrow().val)
            }
        }
        None => false,
    }
}
