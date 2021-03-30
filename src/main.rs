#![feature(cow_is_borrowed)]
#![feature(async_closure)]
#![feature(ptr_internals)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(type_name_of_val)]
#![allow(unused_variables)]
#![feature(test)]
#![feature(in_band_lifetimes)]
#![allow(dead_code)]

mod any;
pub mod array;
pub mod boxes;
pub mod cell;
pub mod clone;
pub mod closures;
pub mod cow;
mod debug_display;
pub mod dequeue;
mod drop;
pub mod enums;
pub mod error;
pub mod flow_control;
mod from_into;
pub mod func;
pub mod generator;
pub mod generic;
pub mod hashmap;
pub mod iter;
pub mod lifetime;
mod macros;
pub mod method;
pub mod option;
mod pin;
pub mod range;
pub mod rc;
mod ref_test;
pub mod refcell;
mod reflect;
pub mod send;
pub mod sendsync;
pub mod str;
pub mod string;
pub mod structs;
pub mod thread;
pub mod traits;
pub mod tuple;
mod unique;
pub mod unsafes;
pub mod var;
pub mod vector;
pub mod weakref;
mod file;
mod bench;
mod ownership;
mod static_;
mod byteiter;
mod atomic;
mod fatpointer;
mod thread_local;
mod trait_object;
mod drop_t;
mod mem;
mod conv;
mod ptr;

fn main() {
}
