use core::fmt;

use crate::uart;

struct Uart {}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            uart::transmit(b);
        }
        Ok(())
    }
}

pub fn _kprint(args: fmt::Arguments) {
    use fmt::Write;

    let mut uart = Uart {};
    uart.write_fmt(args).unwrap();
}

macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::print::_kprint(format_args!($($arg)*))
    }
}
pub(crate) use kprint;
