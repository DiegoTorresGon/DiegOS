use core::arch::asm;
use crate::println;
use lazy_static::lazy_static;

mod idt;

lazy_static! {
    static ref IDT : idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, divide_by_zero_handler);

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
    panic!("DIVISION BY ZERO OCURRED!!!\n{:#?}", stack_frame);
    loop {}
}
