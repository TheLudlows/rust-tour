use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};
/// 不能直接用eq方法
impl Solution {
    pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if a.is_none() || b.is_none() {
            false
        } else {
            eq(a.as_ref(),b.as_ref()) || Self::is_sub_structure(a.as_ref().unwrap().borrow().left.clone(),b.clone())
            || Self::is_sub_structure(a.as_ref().unwrap().borrow().right.clone(),b.clone())
        }
    }
}

fn eq(a:Option<&Rc<RefCell<TreeNode>>>, b: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if b.is_none() {
        true
    } else if a.is_none() || a.unwrap().borrow().val == b.unwrap().borrow().val {
        false
    }else {
        eq(a.unwrap().borrow().left.as_ref(), b.unwrap().borrow().left.as_ref())
            && eq(a.unwrap().borrow().right.as_ref(), b.unwrap().borrow().right.as_ref())
    }
}