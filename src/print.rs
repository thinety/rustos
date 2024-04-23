use core::fmt;

use crate::uart;

struct UartFmt(uart::Uart);

impl fmt::Write for UartFmt {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.0.transmit(b);
        }
        Ok(())
    }
}

pub fn _kprint(args: fmt::Arguments) {
    use fmt::Write;

    let uart = uart::uart();
    UartFmt(uart).write_fmt(args).unwrap();
}

macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::print::_kprint(format_args!($($arg)*))
    }
}
pub(crate) use kprint;
