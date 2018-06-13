#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(const_fn)]
#![feature(allocator_api)]

extern crate psp2_sys;
extern crate vita_mutex;

pub mod debug;
pub mod alloc;
pub mod sync;
