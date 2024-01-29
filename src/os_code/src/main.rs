#![no_std]
#![no_main]
//#![feature(rustc_private)]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(asm_const)]

//extern crate compiler_builtins;

use core::panic::PanicInfo;
use core::arch::asm;

pub mod drivers;
pub mod interrupts;

use drivers::screen::*;
use drivers::screen;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    screen::init_out(RepCode::new(FB_BLACK, FB_WHITE));

    out_handle().clear_screen();

    print!("\t\tDiegOS\n\n");
    out_handle().rep_code = RepCode::new(FB_BLACK, FB_LIGHT_GREY);
    print!("Booting process has started.\n\
            We are initializing some stuff.\n\
            Hold tightly...");

    interrupts::init();

    unsafe {
        x86::int!(0x3);
    };

    panic!("Awwwwgh!!! Horror panic is coming!!!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    out_handle().rep_code = RepCode::new(FB_RED, FB_WHITE);
    match info.message() {
        Some(msg) => {
            print!("\n\nPanic at:\t");
            if info.location().is_some() {
                print!("{}", info.location().unwrap());
            }
            println!("\n\"{}\"", msg);
        },
        None => {
            println!("\n\n{:?}", info);
        }
    }
    loop{}
}

fn _hello_dance() {
    loop {
        for i in 0..15 {
            for j in 0..15 {
                out_handle().rep_code = RepCode::new(i as u8, j as u8);
                println!("Hello world");
            }
        }
    }
}
