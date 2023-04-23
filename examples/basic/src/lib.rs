use shimmer::prelude::*;

#[shimmer]
#[derive(Default)]
struct State {}

trait BasicIO {
    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int;
}

#[shimmer_hook]
impl BasicIO for State {
    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        shimmer_println!("[write] fd={fd}, size={nbytes}");
    }
}
