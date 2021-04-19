use super::version::{version_init};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    printk!("|------------------|\n");
    printk!("| Kernel init      |\n");
    printk!("|------------------|\n");
    // Prints version
    version_init();
}

