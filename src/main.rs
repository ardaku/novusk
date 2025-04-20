#![no_std]
#![no_main]

#[macro_use] extern crate novusk;

#[no_mangle]
pub unsafe extern "Rust" fn kernel_main() {
    printk::printk!("\nKernel Main\n");
}

#[no_mangle]
pub extern "Rust" fn initramfs_main() {

}
