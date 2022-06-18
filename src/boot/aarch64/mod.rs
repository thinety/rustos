use core::arch::asm;
use core::arch::global_asm;

global_asm!(include_str!("./init.s"));

#[no_mangle]
fn main() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}
