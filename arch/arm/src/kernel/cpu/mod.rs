pub mod interrupts;
pub mod irq;

pub fn cpu_init() {
    irq::irq_init();
}
