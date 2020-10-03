use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        same(p.as_ref(),q.as_ref())
    }
}

fn same(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match p {
        Some(p1) => {
            match q {
                Some(q1) => {
                    p1.borrow().val.eq(&q1.borrow().val)
                    && same(p1.borrow().left.as_ref(),q1.borrow().left.as_ref())
                    && same(p1.borrow().right.as_ref(),q1.borrow().right.as_ref())
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