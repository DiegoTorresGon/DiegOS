use core::arch::asm;
use pc_keyboard::DecodedKey;

use crate::drivers::keyboard::keyboard_read;
use crate::interrupts::ExceptionStackFrame;
use crate::interrupts::pic;
use crate::print;
use crate::println;
use crate::screen;


pub extern "x86-interrupt" fn divide_by_zero(
stack_frame : ExceptionStackFrame) -> ! {
    panic!("DIVISION BY ZERO OCURRED!!!\n{:#x?}", stack_frame);
}

pub extern "x86-interrupt" fn breakpoint(
stack_frame : ExceptionStackFrame) {
    let old_color = screen::OutHandler::get_rep_code();
    screen::OutHandler::set_rep_code(
        screen::RepCode::new(screen::FB_BLACK, screen::FB_RED));

    println!("\n\nBreakpoint ocurred at {:#x}:\n{:#x?}",
             stack_frame.instruction_ptr, stack_frame);

    screen::OutHandler::set_rep_code(old_color);

    //magical breakpoint in bochs. This allows me to do debugging in bochs tool.
    unsafe {
        asm!("xchg bx, bx");
    }

    //println!("\nContinuing...");
}


pub extern "x86-interrupt" fn double_fault(
stack_frame : ExceptionStackFrame, error_code : u32) -> ! {
    /*
    unsafe {
        asm!("xchg bx, bx");
    }
    */
    panic!("DOUBLE FAULT OCURRED AT {:#x?}:\n{:#x?}\n{:#x?}",
           stack_frame.instruction_ptr, stack_frame, error_code);
}

//This exception is fired when the Programmable Interval Timer (PIT)
//controller. This acts as the system timer.
pub extern "x86-interrupt" fn sys_timer(_stack_frame : ExceptionStackFrame) {
    //print!(".");
    //For know we are ignoring the timer.
    //Would like to implement time keeping service.

    pic::send_eoi(pic::HardwareInterrupts::Timer.as_u8());
}

pub extern "x86-interrupt" fn keyboard(_stack_frame : ExceptionStackFrame) {
    match keyboard_read() {
        Ok(decoded_key) => match decoded_key {
            DecodedKey::Unicode(character) => print!("{character}"),
            DecodedKey::RawKey(key) => print!("{:?}", key),
        },
        _ => (),
    };

    pic::send_eoi(pic::HardwareInterrupts::Keyboard.as_u8());
}

pub extern "x86-interrupt" fn gpf(stack_frame : ExceptionStackFrame,
                                  error_code : u32) 
-> ! {
    /*
    unsafe {
        asm!("xchg bx, bx");
    }
    */
    panic!("GENERAL PROTECTION FAULT AT {:#x?}:\n{:#x?}\n{:#x?}",
           stack_frame.instruction_ptr, stack_frame, error_code);
}

pub extern "x86-interrupt" fn page_fault(stack_frame : ExceptionStackFrame,
                                  error_code : u32) 
-> ! {
    /*
    unsafe {
        asm!("xchg bx, bx");
    }
    */
    panic!("PAGE FAULT AT {:#x?}:\n{:#x?}\n{:#x?}",
           stack_frame.instruction_ptr, stack_frame, error_code);
}

pub extern "x86-interrupt" fn invalid_opcode(stack_frame : ExceptionStackFrame) 
-> ! {
    panic!("INVALID OP_CODE AT {:#x?}:\n{:#x?}\n",
           stack_frame.instruction_ptr, stack_frame);
}
    

