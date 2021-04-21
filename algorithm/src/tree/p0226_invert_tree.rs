use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        trace(root.as_mut());
        root
    }
}

pub fn trace(node: Option<&mut Rc<RefCell<TreeNode>>>) {
    if node.is_none() {
        return;
    }
    let mut node = node.unwrap().borrow_mut();
    let temp = node.left.take();
    node.left = node.right.take();
    node.right = temp;
    trace(node.left.as_mut());
    trace(node.right.as_mut());
}

#[test]
fn test() {}
