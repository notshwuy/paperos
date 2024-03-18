#![no_std]
#![no_main]

use crate::display::{Display, Color};
use core::panic::PanicInfo;

pub mod display;

pub const SYSTEM_NAME: &[u8] = b"Welcome to PaperOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = Display::create((80, 25), 0xb8000 as *mut u8);

    display.append(Color::Brown as u8, SYSTEM_NAME).ok();

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
