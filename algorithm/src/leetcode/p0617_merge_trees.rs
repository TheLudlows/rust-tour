use std::any::type_name_of_val;
use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::common::{Solution, TreeNode};

impl Solution {
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut t1 = t1;
        mt(&mut t1, &t2);
        t1
    }
}

fn mt(t1: &mut Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(n1) = t1 {
        if let Some(n2) = t2 {
            let mut n1 = n1.borrow_mut();
            let n2 = n2.borrow();
            n1.val += n2.val;
            mt(&mut n1.left, &n2.left);
            mt(&mut n1.right, &n2.right);
        }
    } else {
        *t1 = t2.clone();
    }
}
