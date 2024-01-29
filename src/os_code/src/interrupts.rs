use lazy_static::lazy_static;

mod idt;
mod handlers;


//Allows us to easily pass pointer values to 
//low level IDT entries.
trait Handler {
    fn as_u32(self) -> u32;
}

//General interrupt handler which may 
//handle an error code placed on the stack.
type IntHandlerNoRet = extern "x86-interrupt" fn(_: ExceptionStackFrame) -> !;
type IntHandlerRet = extern "x86-interrupt" fn(_: ExceptionStackFrame);
type IntHandlerNoRetErr = extern "x86-interrupt" fn(_: ExceptionStackFrame,
                                                    _: u32) -> !;

lazy_static! {
    static ref IDT : idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0x0, handlers::divide_by_zero as IntHandlerNoRet);
        //idt.set_handler(0x3, handlers::breakpoint as IntHandlerRet);
        idt.set_handler(0x8, handlers::double_fault as IntHandlerNoRetErr);

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
impl Handler for IntHandlerNoRetErr {
    fn as_u32(self) -> u32 {
        self as u32
    }
}

