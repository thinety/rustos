use core::arch::asm;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}

/// Halts the CPU for _at least_ `n` cycles.
#[inline(always)]
pub fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            asm!("nop");
        }
    }
}
