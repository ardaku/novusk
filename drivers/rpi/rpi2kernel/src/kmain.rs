pub unsafe fn start_kernel_main() {
    extern "Rust" {
        fn kernel_main();
        fn initramfs_main();
    }

    printk!("Starting kernel main...\n");
    initramfs_main();
    kernel_main();
}
