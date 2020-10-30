use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

/// 二叉树中第k小的数
/// 中序遍历
impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut result = Vec::new();
        while !stack.is_empty() || root.is_some() {
            if root.is_some() {
                stack.push(root.clone());
                root = root.unwrap().borrow().left.clone();
            } else {
                let node = stack.pop().unwrap().unwrap();
                result.push(node.borrow().val);
                root = node.borrow().right.clone();
            }
        }
        result[(k - 1) as usize]
    }
}
