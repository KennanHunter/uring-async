use core::arch::asm;

use crate::generated_bindings::{__NR_io_uring_enter, __NR_io_uring_setup, io_uring_params};

struct IoUringBuilder {}

impl IoUringBuilder {
    fn build(queue_depth: usize) -> IoUringWrapper {
        const ENTRIES_COUNT: u32 = 255;
        let mut params = io_uring_params::default();
        let params_ptr = &mut params;

        unsafe {
            let val = io_uring_setup(ENTRIES_COUNT, params_ptr);

            println!("io_uring_setup return: {:?}", val);
            println!("io_uring_setup post-call params struct: {:#?}", params_ptr);
        };

        IoUringWrapper {
            consumer_queue_entries: params.cq_entries,
            submission_queue_entries: params.sq_entries,
        }
    }
}

pub unsafe fn io_uring_setup(entries: u32, params: *mut io_uring_params) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") __NR_io_uring_setup,
            in("rdi") entries as usize,
            in("rsi") params,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }

    ret
}

pub unsafe fn io_uring_enter(ring_fd: i32, to_submit: u32, min_complete: u32, flags: u32) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") __NR_io_uring_enter,
            in("rdi") entries as usize,
            in("rsi") params,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }

    ret
}

#[derive(Debug)]
struct IoUringWrapper {
    consumer_queue_entries: u32,
    submission_queue_entries: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_uring() {
        let ring = IoUringBuilder::build(10);

        println!("{:#?}", &ring);
    }
}
