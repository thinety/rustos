#![feature(panic_info_message)]
#![no_std]

mod cpu;
mod panic;
mod print;

use print::kprint;

#[no_mangle]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) -> ! {
    kprint!("Hello from kernel\n");
    kprint!("x0 = {:?}, x1 = {:?}, x2 = {:?}, x3 = {:?}\n", x0 as *const (), x1, x2, x3);

    panic!("Goodbye from kernel");
}
