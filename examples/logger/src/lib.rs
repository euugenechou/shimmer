use log::info;
use log4rs;
use serde_yaml;
use shimmer::prelude::*;

#[shimmer]
struct State {}

impl Default for State {
    fn default() -> Self {
        let config_str = include_str!("log4rs.yml");
        let config = serde_yaml::from_str(config_str).unwrap();
        log4rs::init_raw_config(config).unwrap();
        Self {}
    }
}

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
        info!("[write] fd={fd}, size={nbytes}");
    }
}
