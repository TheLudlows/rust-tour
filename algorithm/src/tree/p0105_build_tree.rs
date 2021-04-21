use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mid_map = inorder.iter().enumerate().
            fold(HashMap::new(), |mut map, (i, v)| {
                map.insert(*v, i as i32);
                map
            });
        build(&mid_map, &mut preorder.into_iter(), 0, (mid_map.len() - 1) as i32)
    }
}

fn build(mid_map: &HashMap<i32, i32>, pre: &mut dyn Iterator<Item=i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if l > r {
        None
    } else {
        let val = pre.next().unwrap();
        let i = mid_map.get(&val).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: build(mid_map, pre, l, i - 1),
            right: build(mid_map, pre, i + 1, r),
        })))
    }
}
  
