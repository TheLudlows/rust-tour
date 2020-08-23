
#![feature(cow_is_borrowed)]
#![feature(async_closure)]
#![feature(ptr_internals)]
#![feature(generators)]
#![allow(unused_variables)]
mod from_into;
mod debug_display;
mod asyncs;
mod unique;
mod drop;
mod pin;
pub mod func ;
pub mod vector;
pub mod string;
pub mod hashmap;
pub mod error;
pub mod generic;
pub mod traits;
pub mod enums;
pub mod closures;
pub mod iter;
pub mod boxes;
pub mod var;
pub mod rc;
pub mod refcell;
pub mod rcrefcell;
pub mod weakref;
pub mod thread;
pub mod unsafes;
pub mod flow_control;
pub mod array;
pub mod range;
pub mod str;
pub mod tuple;
pub mod structs;
pub mod dequeue;
pub mod heap;
pub mod clone;
pub mod sendsync;
pub mod lifetime;
pub mod cell;
pub mod cow;
pub mod send;
pub mod method;
pub mod option;
fn main() {
}

