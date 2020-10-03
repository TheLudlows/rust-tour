use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => symmetric(node.borrow().right.as_ref(), node.borrow().left.as_ref())
        }
    }
}

fn symmetric(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match p {
        Some(p1) => {
            match q {
                Some(q1) => {
                    p1.borrow().val.eq(&q1.borrow().val)
                        && symmetric(p1.borrow().left.as_ref(), q1.borrow().right.as_ref())
                        && symmetric(p1.borrow().right.as_ref(), q1.borrow().left.as_ref())
                }
                _ => false
            }
        }
        None => {
            match q {
                None => {
                    true
                }
                _ => false
            }
        }
    }
}