use core::fmt::{Arguments, Write};
use super::uart::Uart;

#[export_name = "arch_printk"]
pub extern "Rust" fn _aarch64_printk(fmt: Arguments) {
    let mut uart = Uart::new();

    uart.write_fmt(fmt);
}

#[no_mangle]
pub extern "Rust" fn _kernel_main_print(fmt: Arguments) {
    _aarch64_printk(fmt);
}

#[macro_export]
macro_rules! aarch64_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_aarch64_printk(format_args!($($arg)*))};
}
