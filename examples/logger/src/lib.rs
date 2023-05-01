use log::info;
use shimmer::prelude::*;

#[shimmer]
struct State {}

impl Default for State {
    fn default() -> Self {
        let config = serde_yaml::from_str(include_str!("log4rs.yml")).unwrap();
        log4rs::init_raw_config(config).unwrap();
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
