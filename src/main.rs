#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod cpu;
mod panic;
mod print;

use print::kprint;

#[no_mangle]
fn main() -> ! {
    kprint!("Hello from kernel\n");
    panic!("Goodbye from kernel");
}
