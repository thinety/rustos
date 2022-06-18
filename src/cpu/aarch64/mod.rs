use core::arch::{asm, global_asm};

global_asm!(include_str!("./init.s"));

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}
