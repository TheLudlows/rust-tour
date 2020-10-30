use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        slice_to_bst(nums.as_slice())
    }
}

fn slice_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    let mid = nums.len() / 2;
    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid],
        left: slice_to_bst(&nums[0..mid]),
        right: slice_to_bst(&nums[mid + 1..]),
    })))
}
