use core::panic::PanicInfo;

use crate::cpu::wait_forever;
use crate::print::kprint;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprint!("\nkernel panicked");

    if let Some(arguments) = info.message() {
        kprint!(" at '{}'", arguments);
    }

    if let Some(location) = info.location() {
        kprint!(
            ", {}:{}:{}",
            location.file(),
            location.line(),
            location.column(),
        );
    }

    kprint!("\n");

    wait_forever();
}
