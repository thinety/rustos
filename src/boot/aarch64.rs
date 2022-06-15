use core::arch::asm;

#[link_section = ".text._start"]
#[no_mangle]
fn _start() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}
