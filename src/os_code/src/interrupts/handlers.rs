use crate::println;
use crate::screen;
use crate::interrupts::ExceptionStackFrame;

pub extern "x86-interrupt" fn divide_by_zero(
stack_frame : ExceptionStackFrame) -> ! {
    panic!("DIVISION BY ZERO OCURRED!!!\n{:#x?}", stack_frame);
}

pub extern "x86-interrupt" fn breakpoint(
stack_frame : ExceptionStackFrame) {
    let old_color = screen::out_handle().rep_code;
    screen::out_handle().rep_code =
        screen::RepCode::new(screen::FB_BLACK, screen::FB_RED);

    println!("\n\nBreakpoint ocurred at {:#x}:\n{:#x?}",
             stack_frame.instruction_ptr, stack_frame);

    screen::out_handle().rep_code = old_color;

    println!("\nContinuing...");
}


pub extern "x86-interrupt" fn double_fault(
stack_frame : ExceptionStackFrame, error_code : u32) -> ! {
    panic!("DOUBLE FAULT OCURRED AT {:#x?}:\n{:#x?}\n{:#x?}",
           stack_frame.instruction_ptr, stack_frame, error_code);
}
