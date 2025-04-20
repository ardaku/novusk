#[cfg(not(feature = "custom_config"))]
#[unsafe(no_mangle)]
pub extern "Rust" fn kernel_config() -> &'static str {
    return include_str!("../kernel/konfig/src/defconfig.txt");
}
