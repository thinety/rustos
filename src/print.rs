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

    let mut qemu_output = QEMUOutput {};
    qemu_output.write_fmt(args).unwrap();
}

macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::print::_kprint(format_args!($($arg)*))
    }
}
pub(crate) use kprint;
