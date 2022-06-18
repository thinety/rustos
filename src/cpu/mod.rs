#[cfg_attr(target_arch = "aarch64", path = "aarch64/mod.rs")]
mod arch_cpu;

pub use arch_cpu::wait_forever;
