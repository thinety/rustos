use core::ptr;

use crate::cpu::delay;

const GPFSEL1: *mut u32 = 0x3F20_0004 as _;

const GPPUD: *mut u32 = 0x3F20_0094 as _;
const GPPUDCLK0: *mut u32 = 0x3F20_0098 as _;

const AUXENB: *mut u32 = 0x3F21_5004 as _;
const AUX_MU_IO_REG: *mut u32 = 0x3F21_5040 as _;
const AUX_MU_LCR_REG: *mut u32 = 0x3F21_504C as _;
const AUX_MU_CNTL_REG: *mut u32 = 0x3F21_5060 as _;
const AUX_MU_STAT_REG: *mut u32 = 0x3F21_5064 as _;
const AUX_MU_BAUD_REG: *mut u32 = 0x3F21_5068 as _;

pub fn init() {
    let mut x = unsafe { ptr::read_volatile(GPFSEL1) };

    // set ALT5 for GPIO14
    x &= !(0b111 << 12);
    x |= 0b010 << 12;

    // set ALT5 for GPIO15
    x &= !(0b111 << 15);
    x |= 0b010 << 15;

    unsafe { ptr::write_volatile(GPFSEL1, x) };

    // set control signal (disable pull-up/down)
    unsafe { ptr::write_volatile(GPPUD, 0b00) };
    delay(150);

    // apply control signal to GPIO14 and GPIO15
    unsafe { ptr::write_volatile(GPPUDCLK0, (0b1 << 14) | (0b1 << 15)) };
    delay(150);
    unsafe { ptr::write_volatile(GPPUDCLK0, 0) };

    unsafe {
        // enable auxiliary mini uart
        ptr::write_volatile(AUXENB, 0b1);

        // disable transmitter and receiver (we need to setup first)
        ptr::write_volatile(AUX_MU_CNTL_REG, 0b00);

        // 8-bit mode
        ptr::write_volatile(AUX_MU_LCR_REG, 0b11);

        // 115200 baud rate
        ptr::write_volatile(AUX_MU_BAUD_REG, 270);

        // enable transmitter and receiver
        ptr::write_volatile(AUX_MU_CNTL_REG, 0b11);
    }
}

pub fn receive() -> u8 {
    loop {
        let symbol_available = unsafe { ptr::read_volatile(AUX_MU_STAT_REG) & 0b01 } != 0;
        if symbol_available {
            break;
        }
    }
    unsafe { ptr::read_volatile(AUX_MU_IO_REG) as u8 }
}

pub fn transmit(data: u8) {
    loop {
        let space_available = unsafe { ptr::read_volatile(AUX_MU_STAT_REG) & 0b10 } != 0;
        if space_available {
            break;
        }
    }
    unsafe { ptr::write_volatile(AUX_MU_IO_REG, data as u32) };
}
