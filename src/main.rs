#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    let system_name = b"Hello OS";

    for (character_idx, &character_byte) in system_name.iter().enumerate() {  
        unsafe {
            *vga_buffer.offset(character_idx as isize * 2) = character_byte;
            *vga_buffer.offset(character_idx as isize * 2 + 1) = 0xa;
        }
    }    

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
