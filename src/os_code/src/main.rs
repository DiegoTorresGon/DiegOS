#![no_std]
#![no_main]
//#![feature(rustc_private)]

//extern crate compiler_builtins;

use core::panic::PanicInfo;
use core::fmt::Write;

pub mod drivers;

use drivers::screen::*;

static mut SCREEN_OUT : Option<WriteOut> = None;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    unsafe{
        SCREEN_OUT = Some(WriteOut::new(
            FrameBuffer::default(),
            RepCode::new(FB_BLUE, FB_LIGHT_BROWN)));
    }

    let mut out = unsafe { SCREEN_OUT.as_ref().unwrap().clone() };

    out.clear_screen();
    for _ in 0..10 {
        out.write("hello\n");
        //write!(&mut out, "Hello {}!\n", "world");
    }

    out.write("This is awesome!");
    //panic!("Error");
    //let mut a : Option<i32> = None;
    //a.expect("daf");
    //I think what is going here is the size of the binary cannot be higher
    //than 4 kb for now.


    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    //let mut out = unsafe { SCREEN_OUT.clone() };
    match unsafe { SCREEN_OUT.clone() } {
        Some(mut out) => {
            out.frame_buff.move_cursor(0);
            let _ = write!(&mut out, "{:?}", info);

            loop{}
        },
        None => loop{},
    }
}
