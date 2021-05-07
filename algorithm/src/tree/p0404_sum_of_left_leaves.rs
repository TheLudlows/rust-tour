use std::rc::Rc;
use std::cell::RefCell;
use crate::{TreeNode, Solution};

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum_left(root.as_ref())
    }
}
fn sum_left(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        None => 0,
        Some(node) => {
            if is_leaf(node.borrow().left.as_ref()) {
                //node.borrow().left.borrow().
                node.borrow().left.clone().unwrap().borrow().val + sum_left(node.borrow().right.as_ref())
            } else {
                sum_left(node.borrow().left.as_ref()) + sum_left(node.borrow().right.as_ref())
            }
        }
    }
}
fn is_leaf(node: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match node {
        None => false,
        Some(node) => {
            node.borrow().left.is_none() && node.borrow().right.is_none()
        }
    }
}