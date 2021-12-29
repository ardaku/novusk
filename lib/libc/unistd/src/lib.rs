#![no_std]

pub extern crate novuskinc;

use novuskinc::kernel::syscalls::{arch_syscalls, add_kernel_syscalls};
use novuskinc::kernel::syscalls::table::{DEFAULT_NAME, SYSCALL_TABLE};

#[no_mangle]
pub unsafe extern "C" fn syscall(sys_num: u32, sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    let ret = SYSCALL_TABLE.make_call(sys_num, sys_arg1, sys_arg2, sys_arg3);

    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn add_syscalls() -> i32 {
    arch_syscalls::syscalls_init();
    add_kernel_syscalls();

    if SYSCALL_TABLE.systable_name == DEFAULT_NAME {
        return -1;
    } else { return 1; }
}
