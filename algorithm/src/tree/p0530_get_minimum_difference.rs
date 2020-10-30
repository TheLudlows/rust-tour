use std::cell::RefCell;
use std::i32::MAX;
use std::rc::Rc;

use crate::{Solution, TreeNode};
/// 排序，即中序遍历。逐个比较
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = Vec::new();
        mid_traversal(root.as_ref(), &mut v);
        let mut min = MAX;
        for i in 1..v.len() {
            let dif = v[i] - v[i - 1];
            if dif < min {
                min = dif;
            }
        }
        min
    }
}

fn mid_traversal(node: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(n) = node {
        mid_traversal(n.borrow().left.as_ref(), vec);
        vec.push(n.borrow().val);
        mid_traversal(n.borrow().right.as_ref(), vec);
    }
}

#[test]
fn test() {}
