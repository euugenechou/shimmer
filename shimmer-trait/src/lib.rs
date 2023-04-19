use libc::{c_int, c_void, size_t};

pub trait Shimmer: Default {
    unsafe fn read(&mut self, fd: c_int, buf: *mut c_void, nbytes: size_t) -> c_int;

    unsafe fn write(&mut self, fd: c_int, buf: *mut c_void, nbytes: size_t) -> c_int;
}
