use x86::segmentation::SegmentSelector;
use x86::segmentation;
use x86::dtables::lidt;
use x86::dtables::DescriptorTablePointer;
use x86::Ring;
use core::mem::size_of;

use crate::interrupts::Handler;

//Interrupt Descriptor Table
pub struct Idt([Igd; 16]);

//Interrupt Gate Descriptor
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Igd {
    offset_low : u16,
    gdt_selector: SegmentSelector,
    reserved_zero : u8,
    type_attr : TypeAttributes,
    offset_high : u16,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TypeAttributes(u8);

impl Idt {
    pub fn new() -> Idt {
        Idt([Igd::missing(); 16])
    }

    pub fn load(&self) {
        let descriptor_ptr = DescriptorTablePointer {
            limit : (size_of::<Self>() - 1) as u16,
            base : self as *const _
        };

        unsafe { lidt(&descriptor_ptr) };
    }

    pub fn set_handler
    (&mut self, index : u8, handler : impl Handler) 
    -> &mut TypeAttributes 
    {
        let casted_handler = handler.as_u32();
        self.0[index as usize] = Igd::new(segmentation::cs(), casted_handler);
        &mut self.0[index as usize].type_attr
    }
}

impl Igd {
    pub fn new(gdt_selector : SegmentSelector, handler: u32) -> Igd {
        let handler_ptr : u32 = handler as u32;
        let mut type_attr = TypeAttributes::default();
        type_attr.set_present();
        type_attr.disable_interrupts(true);
        type_attr.set_privilege_level(Ring::Ring0 as u8);

        Igd {
            offset_low : handler_ptr as u16,
            gdt_selector,
            reserved_zero : 0 as u8,
            type_attr,
            offset_high : (handler_ptr >> 16) as u16,
        }
    }

    pub fn missing() -> Igd {
        Igd {
            offset_low : 0,
            gdt_selector: SegmentSelector::new(0, Ring::Ring0),
            reserved_zero : 0,
            type_attr : TypeAttributes::default(),
            offset_high : 0,
        }
    }
}

impl Default for TypeAttributes  {
    //https://wiki.osdev.org/Interrupt_Descriptor_Table
    //meaning of this magicall value
    //Trap gate with P and DPL set to 0.
    //Generate invalid attrutes as
    //P needs to be set to 1 in order 
    //for the IGD to be valid.
    fn default() -> TypeAttributes {
        TypeAttributes(0b1111)
    }
}

impl TypeAttributes {
    pub fn set_present(&mut self) {
        self.0 = self.0 | (0b1 << 7);
    }

    pub fn disable_interrupts(&mut self, disable : bool) {
        self.0 = self.0 & (0b11111110 | (!disable as u8));
    }

    pub fn set_privilege_level(&mut self, dpl : u8) {
        self.0 = self.0 | ( 0b10001111 | ( dpl << 4 ));
    }
}
