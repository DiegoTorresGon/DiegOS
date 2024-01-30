//This file will program the interface
//with the 8259 IBM PC/AT Programmable Interrupt Controller (PIC)
//General reference:
//https://wiki.osdev.org/8259_PIC

use x86::io::{outb, inb};
use crate::interrupts;
use crate::interrupts::{PICM_OFFSET, PICS_OFFSET};
use crate::println;

const PICM : u16 = 0x20;
const PICS : u16 = 0xA0;
const PICM_COMMAND : u16 = PICM;
const PICM_DATA : u16 = PICM + 1;
const PICS_COMMAND : u16 = PICS;
const PICS_DATA : u16 = PICS + 1;

#[repr(u8)]
pub enum HardwareInterrupts {
    Timer = interrupts::PICM_OFFSET,
}

impl HardwareInterrupts {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

pub struct Pic {
    offset : u8,
    command_port : u16,
    data_port : u16,
}

impl Pic {
    pub fn new(offset : u8, command_port : u16, data_port : u16) -> Self {
        Pic {
            offset,
            command_port,
            data_port,
        }
    }

    pub fn does_handle(&self, irq_line : u8) -> bool {
        irq_line >= self.offset && irq_line < self.offset + 8
    }

    pub fn set_masks(&self, masks :u8) {
        unsafe {
        outb(self.data_port, masks);
        }
    }

    pub fn get_masks(&self) -> u8 {
        unsafe {
            inb(self.data_port)
        }
    }
}

pub struct MasterSlavePic {
    pub master : Pic,
    pub slave : Pic,
}

impl MasterSlavePic {
    pub fn new(offset_master : u8, offset_slave : u8) -> Self {
        Self {
            master : Pic::new(offset_master, PICM_COMMAND, PICM_DATA),
            slave : Pic::new(offset_slave, PICS_COMMAND, PICS_DATA),
        }
    }

    //Reference for initialization protocol:
    //https://k.lse.epita.fr/internals/8259a_controller.html
    pub fn init_pics(&self) {
        unsafe {
            //Wait operation, write garbage on port 0x80.
            let wait_op = || outb(0x80, 0);
            let masks_m = self.master.get_masks();
            let masks_s = self.slave.get_masks();

            println!("PIC masks are {:#b}", ((masks_m as u16) << 8) | 
                     (masks_s as u16));

            //First Initialization Command Word.
            //ICW4 present, cascade mode, edge triggered mode
            const ICW1 : u8 = 0x11;
            outb(self.master.command_port, ICW1);
            wait_op();
            outb(self.slave.command_port, ICW1);
            wait_op();


            outb(self.master.data_port, self.master.offset);
            wait_op();
            outb(self.slave.data_port, self.slave.offset);
            wait_op();


            //Slave PIC is on master's IRQ2 
            //0000 0100 
            outb(self.master.data_port, 0x04);
            wait_op();
            outb(self.slave.data_port, 0x02);
            wait_op();

            //Operation mode is sent.
            //0x01 is typical 8086 operation mode.
            //No fully nested mode 0 / Buffering 00 / normal EOI 0 / required 1
            //0x01 = 0b00000001
            const OP_MODE : u8 = 0x01;
            outb(self.master.data_port, OP_MODE);
            wait_op();
            outb(self.slave.data_port, OP_MODE);
            wait_op();

            //Rewriting masks after initialization.
            self.master.set_masks(masks_m);
            self.slave.set_masks(masks_s);
        }
    }
}



pub fn send_eoi(interrupt_number : u8) {
    unsafe{
        const EOI : u8 = 0x20;
        let pics = MasterSlavePic::new(PICM_OFFSET, PICS_OFFSET);

        match (pics.master.does_handle(interrupt_number), 
        pics.slave.does_handle(interrupt_number)) {

            (true, false) => outb(pics.master.command_port, EOI),
            (false, true) => {
                outb(pics.master.command_port, EOI);
                outb(pics.slave.command_port, EOI);
            },
            _ => println!("interrupt_line out of bounds"), 
        };

    }
}










