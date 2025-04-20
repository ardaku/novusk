#![no_std]

#[macro_use] extern crate printk;

use multiboot2::BootInformation;

pub unsafe fn multiboot_init(bootinfo_addr: usize) {
    let mut bootinfo = BootInformation::load(bootinfo_addr as *const _);

    if bootinfo.is_err() {
        panic!("{:?}", bootinfo.err().unwrap());
    }
}
