use std::rc::Rc;
use std::cell::RefCell;
use crate::{Solution, TreeNode};

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut vec = vec![];
        mid_traversal(root, &mut vec)
    }
}

pub fn mid_traversal(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) -> bool {
    match root {
        None => { true }
        Some(node) => {
            let mut ret = true;
            ret = ret && mid_traversal( node.borrow().left.clone(), vec);

            let v = node.borrow().val;
            if !vec.is_empty() {
                if *vec.last().unwrap() >= v {
                    return false;
                }
            }
            vec.push(v);
            ret = ret && mid_traversal( node.borrow().right.clone(), vec);
            ret
        }
    }
}

#[test]
fn test() {

}