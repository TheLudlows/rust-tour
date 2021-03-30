#![feature(type_name_of_val)]
#![feature(ptr_internals)]

use std::cell::RefCell;
use std::rc::Rc;

mod array;
mod backtracking;
pub mod datastruct;
mod dp;
mod hash;
mod heap;
mod lcof;
mod list;
pub mod math;
mod queue;
mod search;
pub mod sort;
mod stack;
mod string;
mod tree;
mod msjd;
mod vec;

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
    pub(crate) fn new_withNext(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
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
