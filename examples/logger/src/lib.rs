use log::*;
use shimmer::prelude::*;
use simple_logger::SimpleLogger;
use std::fs::read_link;
use std::path::PathBuf;

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
        info!("[write] path={} size={nbytes}", fd_to_path(fd));
    }
    unsafe fn read(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        info!("[read] path={} size={nbytes}", fd_to_path(fd));
    }
    unsafe fn pread(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
        offset: libc::off_t,
    ) -> libc::c_int {
        info!("[pread] path={} size={nbytes}", fd_to_path(fd));
    }
    unsafe fn pwrite(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
        offset: libc::off_t,
    ) -> libc::c_int {
        info!("[pwrite] path={} size={nbytes}", fd_to_path(fd));
    }
    unsafe fn pread64(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
        offset: libc::off_t,
    ) -> libc::c_int {
        info!("[pread64] path={} size={nbytes}", fd_to_path(fd));
    }
    unsafe fn pwrite64(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
        offset: libc::off_t,
    ) -> libc::c_int {
        info!("[pwrite64] path={} size={nbytes}", fd_to_path(fd));
    }
}

fn fd_to_path(fd: libc::c_int) -> String {
    let path_fd = PathBuf::from(format!("/proc/self/fd/{}", fd));
    let file_name = read_link(path_fd).unwrap();
    return file_name.to_str().unwrap().to_string();
}
