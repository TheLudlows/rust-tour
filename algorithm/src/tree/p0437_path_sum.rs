use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(node) = root {
            find_sum(Some(node.clone()), sum) +
                Solution::path_sum(node.clone().borrow().left.clone(), sum) +
                Solution::path_sum(node.clone().borrow().right.clone(), sum)
        } else {
            return 0;
        }
    }
}

pub fn find_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    return if let Some(node) = root {
        let mut res = 0;
        if node.borrow().val == sum {
            res += 1;
        }
        res += find_sum(node.clone().borrow().left.clone(), sum - node.borrow().val);
        res += find_sum(node.clone().borrow().right.clone(), sum - node.borrow().val);
        res
    } else {
        0
    }
}

