#![no_std]
#![no_main]
#![feature(rustc_private)]

//extern crate alloc;
extern crate compiler_builtins;
//extern crate rlibc;

use core::panic::PanicInfo;
use core::fmt::Write;

pub mod drivers;

use drivers::screen::*;

static mut screen_out : Option<WriteOut> = None;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    unsafe{
        screen_out = Some(WriteOut::new(
            FrameBuffer::default(),
            RepCode::new(FB_BLUE, FB_LIGHT_BROWN)));
    }

    let mut out = unsafe { screen_out.as_ref().unwrap().clone() };

    out.clear_screen();
    for _ in 0..10 {
        out.write("hello\n");
        //write!(&mut out, "Hello {}!\n", "world");
    }

    out.write("This is awesome!");
    //panic!("Error");


    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    //let mut out = unsafe { screen_out.clone() };
    match unsafe { screen_out.clone() } {
        Some(mut out) => {
            out.frame_buff.move_cursor(0);
            write!(&mut out, "{:?}", info);

            loop{}
        },
        None => loop{},
    }
    loop{}
}
