pub mod amd;
pub mod device;
pub mod intel;

#[cfg(target_arch = "x86_64")]
pub mod ps2_keyboard;

#[cfg(target_arch = "x86_64")]
pub mod ps2_mouse;

#[cfg(target_arch = "x86_64")]
pub mod task;

#[cfg(target_arch = "x86_64")]

pub use uefi_kd::{print::print_uefi_init, init::uefi_init};
