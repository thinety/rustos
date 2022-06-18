use core::{fmt, ptr};

struct QEMUOutput {}

impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe {
                ptr::write_volatile(0x3F20_1000 as *mut _, b);
            }
        }
        Ok(())
    }
}

pub fn _kprint(args: fmt::Arguments) {
    use fmt::Write;
    { QEMUOutput {} }.write_fmt(args).unwrap();
}

pub macro kprint {
    ($($arg:tt)*) => {
        _kprint(format_args!($($arg)*))
    }
}
