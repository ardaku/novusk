extern "Rust" {
    pub(crate) fn device_specific_irq_init();
}

#[macro_export]
macro_rules! define_dev_irq_init {
    ($init_fun:ident) => {
        #[no_mangle]
        pub extern "Rust" fn device_specific_irq_init() {
            unsafe { $init_fun(); }
        }
    };
}

pub fn irq_error(irq_name: &str, message: &str) -> ! {
    /*printk!("An error occurred while running IRQ: {}\n", irq_name);
    printk!("{}\n", message);*/

    panic!("Interrupt error occurred while running {} interrupt", irq_name);
}
