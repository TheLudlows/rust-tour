use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut v = Vec::new();
        traversal(root.as_ref(), &mut v);
        v[(k - 1) as usize]
    }
}
fn traversal(node: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(n) = node {
        traversal(n.borrow().right.as_ref(), vec);
        vec.push(n.borrow().val);
        traversal(n.borrow().left.as_ref(), vec);
    }
}
