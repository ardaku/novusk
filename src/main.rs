#![feature(asm, global_asm)]
#![no_std]
#![no_main]

mod kprint;
mod panic;

extern crate arch;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    kprint!("Kernel init\n");
    unsafe { kernel_main() }
}

unsafe fn kernel_main() -> ! {
    loop {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        asm!("hlt");
    }
}
