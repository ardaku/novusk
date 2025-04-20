pub fn blink(sleep_time: usize) {
    extern "Rust" { fn led_blink(sleep: usize); }

    unsafe { led_blink(sleep_time); }
}
