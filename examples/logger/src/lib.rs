use log::*;
use shimmer::prelude::*;
use simple_logger::SimpleLogger;

#[shimmer]
struct State {}

impl Default for State {
    fn default() -> Self {
        SimpleLogger::new().init().unwrap();
        log::set_max_level(LevelFilter::Info);
        Self {}
    }
}

#[shimmer_hook]
impl State {
    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        info!("[write] fd={fd}, size={nbytes}");
    }
}
