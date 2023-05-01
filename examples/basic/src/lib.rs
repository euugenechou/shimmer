use shimmer::prelude::*;

#[shimmer]
#[derive(Default)]
struct State {}

#[shimmer_hook]
impl State {
    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        shimmer_println!("[write] fd={fd}, size={nbytes}");
    }
}
