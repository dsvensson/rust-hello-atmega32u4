#![feature(lang_items)]
#![feature(asm)]
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}

pub const F_CPU: u32 = 16_000_000 as u32;

pub const DELAY_MS: u16 = (F_CPU / 4000) as u16;

pub const DDRB:  *mut u16 = 0x24 as *mut u16;
pub const PORTB:  *mut u16 = 0x25 as *mut u16;
pub const DDRD:  *mut u16 = 0x2a as *mut u16;
pub const PORTD:  *mut u16 = 0x2b as *mut u16;

// http://www.nongnu.org/avr-libc/user-manual/group__util__delay__basic.html
#[inline(always)]
pub fn _delay_loop_2(count: u16) {
    unsafe {
        let mut __count = count;
        asm!("1: sbiw $0, 1
                 brne 1b"
             : "=w"(__count)
             : "0"(__count)
             :
             : "volatile")
    }
}

pub fn delay_ms(ms: u16) {
    for _ in 0..ms {
        _delay_loop_2(DELAY_MS);
    }
}

pub fn enable(addr: *mut u16, bit: u16) {
    unsafe {
        write_volatile(addr, read_volatile(addr) | bit)
    }
}

pub fn disable(addr: *mut u16, bit: u16) {
    unsafe {
        write_volatile(addr, read_volatile(addr) & !bit)
    }
}

#[no_mangle]
pub extern fn main() {
    enable(DDRD, 1 << 5);
    enable(DDRB, 1 << 0);

    loop {
        enable(PORTB, 1 << 0);
        disable(PORTD, 1 << 5);
        delay_ms(2000);

        disable(PORTB, 1 << 0);
        enable(PORTD, 1 << 5);
        delay_ms(2000);
    }
}
