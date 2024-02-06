#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![feature(asm_const)]
#![feature(ptr_as_uninit)]

//extern crate compiler_builtins;

use core::panic::PanicInfo;

pub mod drivers;
pub mod interrupts;
pub mod mem;

use drivers::screen::*;
use drivers::screen;
use mem::paging;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    screen::init_out(RepCode::new(FB_BLACK, FB_WHITE));
    interrupts::init();
    paging::init();

    OutHandler::clear_screen();

    print!("\t\tDiegOS\n\n");
    OutHandler::set_rep_code(RepCode::new(FB_BLACK, FB_LIGHT_GREY));
    print!("Booting process has started.\n\
            We are initializing some stuff.\n\
            Hold tightly...\n");
    /*
    unsafe {
        x86::int!(0x3);
    };
    */

    println!("Succesfully initialized so far...");


    panic!("Awwwwgh!!! Horror panic is coming!!!");
    //interrupts::halt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    OutHandler::set_rep_code(RepCode::new(FB_RED, FB_WHITE));
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
    
    interrupts::halt();
}

fn _hello_dance() {
    loop {
        for i in 0..15 {
            for j in 0..15 {
                OutHandler::set_rep_code(RepCode::new(i as u8, j as u8));
                println!("Hello world");
            }
        }
    }
}
