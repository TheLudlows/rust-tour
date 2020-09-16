use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::common::{TreeNode, Solution};

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        traversal(root, &mut v);
        v
    }
}

fn traversal(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(n) = node {
        traversal(n.borrow().left.clone(), vec);
        vec.push(n.borrow().val);
        traversal(n.borrow().right.clone(), vec);
    }
}

#[test]
fn test() {

}