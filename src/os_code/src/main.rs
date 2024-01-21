#![no_std]
#![no_main]
//#![feature(rustc_private)]
#![feature(panic_info_message)]

//extern crate compiler_builtins;

use core::panic::PanicInfo;
use core::fmt::Write;

pub mod drivers;

use drivers::screen::*;
use drivers::screen;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    screen::init_out(RepCode::new(FB_BLACK, FB_WHITE));

    out_handle().clear_screen();

    print!("\t\tDiegOS\n\n");
    out_handle().rep_code = RepCode::new(FB_BLACK, FB_LIGHT_GREY);
    print!("Booting proces has started.\n\
            We are initializing some stuff.\n\
            Hold tightly...");

    panic!("Awwwwgh!!! Horror panic is coming!!!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    out_handle().rep_code = RepCode::new(FB_RED, FB_WHITE);
    match info.message() {
        Some(msg) => {
            println!("\n\nPanic at:\t{}", info.location().unwrap());
            println!("\"{}\"", msg);
        },
        None => {
            println!("\n\n{:?}", info);
        }
    }
    loop{}
}

fn hello_dance() {
    loop {
        for i in 0..15 {
            for j in 0..15 {
                out_handle().rep_code = RepCode::new(i as u8, j as u8);
                println!("Hello world");
            }
        }
    }
}
