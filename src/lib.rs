pub use shimmer_macro::{shimmer, shimmer_hook};

#[macro_export]
macro_rules! shimmer_print {
    ($($args:tt)*) => {{
        unsafe {
            let mut msg = format!($($args)*);
            libc::syscall(
                #[cfg(target_os = "macos")]
                4,
                #[cfg(not(target_os = "macos"))]
                libc::SYS_write,
                libc::STDOUT_FILENO,
                msg.as_bytes().as_ptr(),
                msg.as_bytes().len(),
            );
        };
    }};
}

#[macro_export]
macro_rules! shimmer_println {
    () => {{
        unsafe {
            let msg = "\n";
            libc::syscall(
                #[cfg(target_os = "macos")]
                4,
                #[cfg(not(target_os = "macos"))]
                libc::SYS_write,
                libc::STDOUT_FILENO,
                msg.as_bytes().as_ptr(),
                msg.as_bytes().len(),
            );
        };
    }};
    ($($args:tt)*) => {{
        unsafe {
            let msg = format!($($args)*) + "\n";
            libc::syscall(
                #[cfg(target_os = "macos")]
                4,
                #[cfg(not(target_os = "macos"))]
                libc::SYS_write,
                libc::STDOUT_FILENO,
                msg.as_bytes().as_ptr(),
                msg.as_bytes().len(),
            );
        };
    }};
}

#[macro_export]
macro_rules! shimmer_eprint {
    ($($args:tt)*) => {{
        unsafe {
            let mut msg = format!($($args)*);
            libc::syscall(
                #[cfg(target_os = "macos")]
                4,
                #[cfg(not(target_os = "macos"))]
                libc::SYS_write,
                libc::STDERR_FILENO,
                msg.as_bytes().as_ptr(),
                msg.as_bytes().len(),
            );
        };
    }};
}

#[macro_export]
macro_rules! shimmer_eprintln {
    () => {{
        unsafe {
            let msg = "\n";
            libc::syscall(
                #[cfg(target_os = "macos")]
                4,
                #[cfg(not(target_os = "macos"))]
                libc::SYS_write,
                libc::STDERR_FILENO,
                msg.as_bytes().as_ptr(),
                msg.as_bytes().len(),
            );
        };
    }};
    ($($args:tt)*) => {{
        unsafe {
            let msg = format!($($args)*) + "\n";
            libc::syscall(
                #[cfg(target_os = "macos")]
                4,
                #[cfg(not(target_os = "macos"))]
                libc::SYS_write,
                libc::STDERR_FILENO,
                msg.as_bytes().as_ptr(),
                msg.as_bytes().len(),
            );
        };
    }};
}

pub mod prelude {
    pub use crate::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[shimmer]
    struct State {
        arg1: usize,
        arg2: usize,
    }

    impl Default for State {
        fn default() -> Self {
            Self { arg1: 45, arg2: 13 }
        }
    }

    impl State {
        fn increment_arg1(&mut self) {
            self.arg1 += 1;
        }

        fn increment_arg2(&mut self) {
            self.arg2 += 2;
        }
    }

    #[shimmer_hook]
    impl State {
        unsafe fn read(
            &mut self,
            fd: libc::c_int,
            buf: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::c_int {
            self.increment_arg1();
        }

        unsafe fn write(
            &mut self,
            fd: libc::c_int,
            buf: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::c_int {
            self.increment_arg2();
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(SHIMMER_SHARED_STATE.lock().unwrap().arg1, 45);
        assert_eq!(SHIMMER_SHARED_STATE.lock().unwrap().arg2, 13);
    }
}
