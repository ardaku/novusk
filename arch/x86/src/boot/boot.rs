use core::fmt::Arguments;
use nkuefi::kernel::system_table;
use crate::drivers::vga::{VGA_ADDRESS, init::vga_init};
use crate::include::asm::hlt;
use crate::kernel::kernel::*;

extern "C" { pub fn boot_method() -> &'static str; }

pub static mut BOOT: &'static str = "";

pub unsafe fn die() -> ! {
    loop { hlt(); }
}

#[no_mangle]
pub unsafe extern "C" fn boot_init() {
    BOOT = boot_method();
    if BOOT == "BIOS" {
        bios_setup();
    } else if BOOT == "UEFI" {
        uefi_setup();
    } else {  }
}

unsafe fn bios_setup() {
    vga_init(25, 80, 0xb8000);
}

unsafe fn uefi_setup() {
    let uefi_version = system_table().as_ref().uefi_revision();
    let major_version = uefi_version.major();
    let minor_version = uefi_version.minor();

    if major_version < 2 {
        x86_printk!("UEFI version is: {}, required is 2+", major_version);
        x86_printk!("Your device is unsupported");
    } else if minor_version < 30 {
        x86_printk!("UEFI minor version is: {}, needed is 30+", minor_version);
        x86_printk!("Some features may not be supported");
    } else {  }
}