#![no_std]
#![no_builtins]
#![no_main]

//extern crate alloc;

use core::panic::PanicInfo;

pub mod drivers;

use drivers::screen::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    let mut out = WriteOut::new(
        FrameBuffer::default(),
        RepCode::new(FB_BLUE, FB_LIGHT_BROWN));

    for _ in 0..60 {
        out.write("Hello World!\n");
    }

    out.write("This is fucking awesome!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
