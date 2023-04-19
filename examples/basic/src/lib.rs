use libc::{c_int, size_t};
use shimmer::prelude::*;

#[shimmer]
#[derive(Default)]
struct State {}

#[shimmer_hook]
impl Shimmer for State {
    unsafe fn read(&mut self, fd: c_int, buf: *mut std::ffi::c_void, nbytes: size_t) -> c_int {
        println!("[read] fd={fd}, size={nbytes}");
    }

    unsafe fn write(&mut self, fd: c_int, buf: *mut std::ffi::c_void, nbytes: size_t) -> c_int {
        println!("[write] fd={fd}, size={nbytes}");
    }
}
