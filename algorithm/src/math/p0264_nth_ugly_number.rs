use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap};

use crate::Solution;

impl Solution {
    // dp
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers = vec![1; n as usize];
        let (mut i2, mut i3, mut i5) = (0, 0, 0);
        for i in 1..(n as usize) {
            let (x2, x3, x5) = (
                2 * ugly_numbers[i2],
                3 * ugly_numbers[i3],
                5 * ugly_numbers[i5],
            );
            let ugly = min(x2, min(x3, x5));
            ugly_numbers[i] = ugly;
            if ugly == x2 {
                i2 += 1;
            }
            if ugly == x3 {
                i3 += 1;
            }
            if ugly == x5 {
                i5 += 1;
            }
        }
        ugly_numbers[(n - 1) as usize]
    }
}

// 最小堆方式, 注意 rust 默认是大顶堆，Java默认是小顶堆
pub fn nth_ugly_number(n: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut set = std::collections::HashSet::new();
    let mut vec = vec![];
    set.insert(1);
    heap.push(Reverse(1));
    let fact: Vec<u64> = vec![2, 3, 5];
    let n = n as u64;
    for _ in 0..n {
        let cur = heap.pop().unwrap().0;
        vec.push(cur);
        for n in fact.iter() {
            let add = n * cur;  // overflow u64
            if set.insert(add) {
                heap.push(Reverse(add));
            }
        }
    }
    //println!("{:?}", vec);
    *vec.last().unwrap() as i32
}

#[test]
fn test() {
    //assert_eq!(nth_ugly_number(10), 12);
    // nth_ugly_number(1407);
    println!("{}", nth_ugly_number(1500))
}