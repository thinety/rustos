use core::arch::asm;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}
