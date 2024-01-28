use crate::println;
use lazy_static::lazy_static;
use crate::screen;

mod idt;

trait Handler {
    fn as_u32(self) -> u32;
}

//General interrupt handler which may 
//handle an error code placed on the stack.
type IntHandlerNoRet = extern "x86-interrupt" fn(_: ExceptionStackFrame) -> !;
type IntHandlerRet = extern "x86-interrupt" fn(_: ExceptionStackFrame);
impl Handler for IntHandlerNoRet {
    fn as_u32(self) -> u32 {
        self as u32
    }
}
impl Handler for IntHandlerRet {
    fn as_u32(self) -> u32 {
        self as u32
    }
}



lazy_static! {
    static ref IDT : idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0x0, divide_by_zero_handler as IntHandlerNoRet);
        idt.set_handler(0x3, breakpoint_handler as IntHandlerRet);

        idt
    };
}

#[repr(C)]
#[derive(Debug)]
struct ExceptionStackFrame {
    instruction_ptr : u32,
    code_segment : u32,
    cpu_flags : u32,
    stack_ptr : u32,
    stack_segment : u32,
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn divide_by_zero_handler(
stack_frame : ExceptionStackFrame) -> ! {
    panic!("DIVISION BY ZERO OCURRED!!!\n{:#x?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(
stack_frame : ExceptionStackFrame) {
    let old_color = screen::out_handle().rep_code;
    screen::out_handle().rep_code =
        screen::RepCode::new(screen::FB_BLACK, screen::FB_RED);

    println!("\n\nBreakpoint ocurred at {:#x}:\n{:#x?}",
             stack_frame.instruction_ptr, stack_frame);

    screen::out_handle().rep_code = old_color;

    println!("\nContinuing...");
}
