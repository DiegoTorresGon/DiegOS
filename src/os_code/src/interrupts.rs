use lazy_static::lazy_static;
use core::arch::asm;

mod idt;
mod handlers;
mod pic;

pub const PICM_OFFSET : u8 = 0x20;
pub const PICS_OFFSET : u8 = 0x28;

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
        idt.set_handler(0x3, handlers::breakpoint as IntHandlerRet);
        idt.set_handler(0x8, handlers::double_fault as IntHandlerNoRetErr);
        idt.set_handler(0x0d, handlers::gpf as IntHandlerNoRetErr);
        idt.set_handler(0x0e, handlers::page_fault as IntHandlerNoRetErr)
            .disable_interrupts(true);
        idt.set_handler(pic::HardwareInterrupts::Timer.as_u8(), 
                        handlers::sys_timer as IntHandlerRet)
            .disable_interrupts(true);
        idt.set_handler(pic::HardwareInterrupts::Keyboard.as_u8(), 
                        handlers::keyboard as IntHandlerRet)
            .disable_interrupts(true);
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
    let pic_handle = pic::MasterSlavePic::new(PICM_OFFSET, PICS_OFFSET);
    //as pin 02 in master is masked, everything in slave pic is masked
    pic_handle.master.set_masks(0b11111100);
    pic_handle.slave.set_masks(0b10001111);
    pic_handle.init_pics();
    enable_hardware_interrupts();
}

pub fn enable_hardware_interrupts() {
    unsafe {
        asm!("sti");
    }
}
 
pub fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}

pub fn without_interrupts<F, R>(f : F) -> R
where 
    F : FnOnce() -> R,
{
    unsafe {
        asm!("cli");
        let value = f();
        asm!("sti");
        value
    }
}

pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
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

