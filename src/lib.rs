#![no_std]
#![needs_allocator]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(alloc)]
#![feature(const_fn)]
#![feature(allocator_api)]
#![feature(allocator_internals)]

extern crate alloc;
extern crate psp2_sys;

pub mod debug;
pub mod sync;
