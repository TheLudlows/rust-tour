#![feature(type_name_of_val)]
#![feature(ptr_internals)]

use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
mod array;
#[allow(dead_code)]
mod backtracking;
#[allow(dead_code)]
mod dp;
#[allow(dead_code)]
mod hash;
#[allow(dead_code)]
mod heap;
#[allow(dead_code)]
mod lcof;
#[allow(dead_code)]
mod list;
#[allow(dead_code)]
mod math;
#[allow(dead_code)]
mod queue;
#[allow(dead_code)]
mod search;
#[allow(dead_code)]
mod sort;
#[allow(dead_code)]
mod stack;
#[allow(dead_code)]
mod string;
#[allow(dead_code)]
mod tree;
#[allow(dead_code)]
mod msjd;
#[allow(dead_code)]
mod slide_window;
#[allow(dead_code)]
mod double_pointer;
#[allow(dead_code)]
mod dc;
#[allow(dead_code)]
mod data_struct;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn as_list(arr: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut curr = &mut head;
    for i in arr {
        curr.next = Some(Box::new(ListNode::new(*i)));
        curr = curr.next.as_mut().unwrap();
    }
    head.next
}

#[test]
fn test() {
    let v = vec![1, 2, 3];
    let head = as_list(&v);
    println!("{:?}", head);
}

pub struct Solution;
