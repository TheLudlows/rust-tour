use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::common::{Solution, TreeNode};

impl Solution {
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let None = root {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        let root_ref = root.as_mut();
        trace(root_ref,val);
        root
    }
}

fn trace(node: Option<&mut Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(n) = node {
        let mut node_ref = n.borrow_mut();
        let v = node_ref.val;
        if v > val{
            let left = node_ref.left.as_mut();
            if let None  = left {
                node_ref.left = Some(Rc::new(RefCell::new(TreeNode::new(val))))
            } else {
                trace(node_ref.left.as_mut(),val)
            }
        } else if v < val{
            let right =node_ref.right.as_mut();
            if let None  = right {
                node_ref.right = Some(Rc::new(RefCell::new(TreeNode::new(val))))
            } else {
                trace(node_ref.right.as_mut(),val)
            }
        }
    }
}
#[test]
fn test() {

}