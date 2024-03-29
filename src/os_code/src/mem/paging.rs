
use core::convert::From;
use core::ops::Deref;
use core::arch::asm;

use crate::interrupts;
use crate::println;
//For referece about this implementation of paging: https://wiki.osdev.org/Paging#32-bit_Paging_.28Protected_Mode.29
//Contol registers reference: https://wiki.osdev.org/CPU_Registers_x86#Control_Registers

// We want to set up 32-bit paging with 4KByte pages.
// According to intel IA-32 architecture developer manual, we must set up like this:

// CR0.PG = 1 Enables paging. Must have protection enable CR0.PE = 1.
// CR4.PAE = 0 uses 32-bit paging mode,
// IA32_EFER.LME = 0. This is ensured by the processor when CR0.PG = 1 && CR4.PAE = 0.
// CR4.PSE = 0 to use 4kiB pages.
// CR0.WP also gets enabled by this code so we can have hardware level 
// memory protection.

const NUM_PTS : usize = 9;
const PDT_ADDR : u32 = 0x76000;
const PTS_ADDR : u32 = PDT_ADDR + TABLE_SIZE;
const TABLE_SIZE : u32 = 4096; //Size in bytes of tables

type PtArray = [PageTable; NUM_PTS];

//These would be extremely unsafe
//but there is no problem as this is kernel code managing memory directly. 
//There is an external map of memory with this space occupied.
static mut PDT : *mut PageDirectoryTable = 
    PDT_ADDR as *mut PageDirectoryTable;

static mut PTS : *mut PtArray = PTS_ADDR as *mut PtArray; 

pub static mut LOADED_PAGE_TABLES : u32  = 0;


//#[repr(align(4096))]
pub struct PageDirectoryTable  {
    pub pde : [PDEMemRep; 1024]
}


//#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PageTable {
    pub page_frames : [PageFrameMemRep ; 1024]
}

#[derive(Clone, Copy)]
//#[repr(align(4096))]
pub struct PageDirectoryEntry {
    address : u32, //Physical address of page table referenced, Bits 31:12
    avl : u8,  //This 3 bit field can be used freely by the Kernel. Bit 11:8
    ps : bool,  //must be 0 for 4kiB pages Bit 7
    avl2 : bool, //Can be used freely. Bit 6
    accessed : bool, //Indicates if page table has been used. Bit 5
    cannot_cache : bool, //If set, page will not be chached. Bit 4
    pwt : bool, //If set, write-through caching enabled, else write-back enabled. Bit 3
    u_s : bool, //If set to 0, user mode access is not enabled. Bit 2
    r_w : bool, //If set, writes are enabled. Bit 1
    p : bool, //Present. Bit 0
}

//#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PDEMemRep(u32);

#[derive(Clone, Copy)]
//#[repr(align(4096))]
pub struct PageFrame {
    address : u32, //Physical address of 4KByte page referenced. Bits 31:12
    avl : u8, //Ignored. Bits 11:9
    g : bool, //Indicates if translation chached should be global. Bit 8
    pat : bool, //Should be set only if processor supports PAT. Bit 7
    dirty : bool, //Indicates if page has been written to. Bit 6
    accessed : bool, //Indicates if it has been accessed. Bit 5
    pcd : bool, //Page level chache disable. Bit 4
    pwt : bool, //Same as in PageTable. Bit 3
    u_s : bool, //Sames as in PageTable. Bit 2
    r_w : bool, //If set, writes are enabled. Bit 1
    p : bool, //Present. Bit 0
}

//#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PageFrameMemRep(u32);

#[derive(Clone, Copy, Debug)]
pub struct VirtAddress(u32);

pub struct PageTableLocator(());

pub fn mem_init_tables() {
    unsafe {
        //x86::int!(0x3);
        let directory_raw = PDT_ADDR as *mut PageDirectoryTable;
        //directory_raw.write_bytes(0, TABLE_SIZE as usize);
        directory_raw.read();

        let tables_raw : *mut PtArray = (PDT_ADDR + TABLE_SIZE) as *mut PtArray;
        //tables_raw.write_bytes(0, TABLE_SIZE as usize);
        tables_raw.read();
        //x86::int!(0x3);
    }
}

impl VirtAddress {
    //10 bit offset
    pub fn dir_offset(&self) -> usize {
        (self.0 >> 22) as usize
    }

    //10 bit offset
    pub fn frame_index(&self) -> usize {
        let res = ((self.0 & 0b0000000000_1111111111_000000000000) >> 12) 
            as usize;
        res
    }

    //12 bit final offset
    pub fn offset(&self) -> u32 {
        (self.0 & 0b0000000000_0000000000_111111111111) as u32
    }

    pub fn to_physical(&self, page_directory : &PageDirectoryTable) -> u32 {
        unsafe {
            PageFrame::from(
                (*(PageDirectoryEntry::from(
                                PDEMemRep::from(page_directory.pde[self.dir_offset()]))
                    .address as *const PageTable))
                .page_frames[self.frame_index()]).address + self.offset()
        }
    }
}

impl PageDirectoryEntry {
    pub fn null() -> Self {
        Self {
            address : 0,
            avl : 0,
            ps : false,
            avl2 : false,
            accessed : false,
            cannot_cache : false,
            pwt : true,
            u_s : false,
            r_w : false,
            p : false,
        }
    }

    pub fn is_present(&self) -> bool {
        self.p
    }
}

impl Deref for PDEMemRep {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for PDEMemRep {
    fn from(val : u32) -> Self {
        PDEMemRep(val)
    }
}

impl From<PageDirectoryEntry> for PDEMemRep {
    fn from(val : PageDirectoryEntry) -> Self {
        PDEMemRep(
            (val.address) | ((val.avl as u32) << 8) |
            ((val.ps as u32) << 7) | ((val.avl2 as u32) << 6) |
            ((val.accessed as u32) << 5) | ((val.cannot_cache as u32) << 4) |
            ((val.pwt as u32) << 3) | ((val.u_s as u32) << 2) |
            ((val.r_w as u32) << 1) | (val.p as u32)
        )
    }
}

impl From<PageDirectoryEntry> for u32 {
    fn from(val : PageDirectoryEntry) -> Self {
        PDEMemRep::from(val).0
    }
}

impl From<PDEMemRep> for PageDirectoryEntry {
    fn from(mem_rep : PDEMemRep) -> Self {
        PageDirectoryEntry {
            address : *mem_rep & 0b11111111111111111111000000000000,
            avl : ((*mem_rep & 0x00ff0000) >> 8) as u8,
            ps : ((*mem_rep & 0b00000000000000000000000010000000) >> 7) == 1,
            avl2 : ((*mem_rep & 0b00000000000000000000000001000000) >> 6) == 1,
            accessed : 
                ((*mem_rep & 0b00000000000000000000000000100000) >> 5) == 1,
            cannot_cache : 
                ((*mem_rep & 0b00000000000000000000000000010000) >> 4) == 1,
            pwt : ((*mem_rep & 0b00000000000000000000000000001000) >> 3) == 1,
            u_s : ((*mem_rep & 0b00000000000000000000000000000100) >> 2) == 1,
            r_w : ((*mem_rep & 0b00000000000000000000000000000010) >> 1) == 1,
            p : ((*mem_rep & 0b00000000000000000000000000000001) >> 1) == 1,
        }
    }
}

impl PageFrame {
    pub fn null() -> PageFrame {
        PageFrame {
            address : 0,
            avl : 0,
            g : false,
            pat : false,
            dirty : false,
            accessed : false,
            pcd : false,
            pwt : true,
            u_s : false,
            r_w : false,
            p : false,
        }
    }
}

impl Deref for PageFrameMemRep {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PageFrame> for PageFrameMemRep {
    fn from(val : PageFrame) -> Self {
        PageFrameMemRep(
            (val.address) | ((val.avl as u32) << 9) |
            ((val.g as u32) << 8) | ((val.pat as u32) << 7) | 
            ((val.dirty as u32) << 6) | ((val.accessed as u32) << 5) | 
            ((val.pcd as u32) << 4) | ((val.pwt as u32) << 3) | 
            ((val.u_s as u32) << 2) | ((val.r_w as u32) << 1) | 
            (val.p as u32)
        )
    }

}

impl From<PageFrameMemRep> for PageFrame {
    fn from(mem_rep : PageFrameMemRep) -> Self {
        println!("{:x}", *mem_rep);
        PageFrame {
            address : *mem_rep & 0b11111111111111111111000000000000,
            avl : ((*mem_rep & 0b00000000000000000000111000000000) >> 9) as u8,
            g : ((*mem_rep & 0b00000000000000000000000100000000) >> 8) == 1,
            pat : ((*mem_rep & 0b00000000000000000000000010000000) >> 7) == 1,
            dirty : ((*mem_rep & 0b00000000000000000000000001000000) >> 6) == 1,
            accessed : 
                ((*mem_rep & 0b00000000000000000000000000100000) >> 5) == 1,
            pcd : ((*mem_rep & 0b00000000000000000000000000010000) >> 4) == 1,
            pwt : ((*mem_rep & 0b00000000000000000000000000001000) >> 3) == 1,
            u_s : ((*mem_rep & 0b00000000000000000000000000000100) >> 2) == 1,
            r_w : ((*mem_rep & 0b00000000000000000000000000000010) >> 1) == 1,
            p : ((*mem_rep & 0b00000000000000000000000000000001) >> 1) == 1,
        }
    }
}

pub fn init() {
    mem_init_tables();
    unsafe {
        // We are identity mapping the first MiB,
        // that is, 256 page frames.
        let ident_start_frame = VirtAddress(0x0);
        let ident_end_frame = VirtAddress(255 * 4096);

        //map_table(ident_address, 0x0);
        map_pages(ident_start_frame, ident_end_frame, 0x0); 
        println!("Succesfully mapped first MiB");

        // TO DO: move kernel code to 0x100000;

        //Map physical addresses 0x100000 - 0xEFFFFF
        //to virtual 0xC0100000 - 0xC0EFFFFF
        let start_addr = VirtAddress(0xC0100000);
        let final_addr = VirtAddress(0xC0CFF000);

        map_pages(start_addr, final_addr, 0x00100000);

        println!("Finished mapping...");
        enable_paging();
    }
}

//#[no_mangle]
unsafe fn enable_paging() {
    //For reference about control registers manipulation
    //see top of the file.
    let pdt_ref = PDT_ADDR as usize;
    println!("{:x}", pdt_ref);
    

    println!("Ready to enable paging...");
    interrupts::without_interrupts(|| {
        asm!(
            "mov eax, cr3",
            "and eax, 0x00000FFF",
            "or eax, edx",
            "or eax, 00000000000000000000000000001000b",
            "mov cr3, eax",

            "mov eax, cr4",
            "or eax, 00000000000000000000000010000000b",
            "and eax, 11111111111111111111111111001111b",
            "mov cr4, eax",

            "mov eax, cr0",
            "or eax, 10000000000000010000000000000001b",
            "mov cr0, eax",
            "nop",

            in("edx") pdt_ref
        );

        println!("Page setup succesfull!");
    });
}

// maps a full table
// 4 MiB = 1024 page frames * 4 KiB per page frame
unsafe fn _map_table(start : VirtAddress, phys_start : u32) {
    map_pages(start, VirtAddress(start.0 + (1023 * 4096) as u32), phys_start);
}

//This maps pages, end is the virtual address of the last page to map
//Maps pages [start, start + 4KiB, ..., end]
//start and end should be 4KiB aligned.
unsafe fn map_pages(start : VirtAddress, end : VirtAddress, phys_start : u32) {
    let mut current_virtual = start;
    let mut current_phys : u32 = phys_start;
    let mut current_table = LOADED_PAGE_TABLES as usize;

    while current_virtual.0 <= end.0  { 
        (*PTS)[current_table]
        .page_frames[current_virtual.frame_index()] = PageFrame {
            address : current_phys,
            avl : 0x0,
            g : true,
            pat : false,
            dirty : false,
            accessed : false,
            pcd : false,
            pwt : true,
            u_s : true,
            r_w : true,
            p : true,
        }.into();

        current_virtual.0 += 4096;
        current_phys += 4096;
        current_table = current_virtual.dir_offset() - start.dir_offset();
    }

    current_virtual.0 -= 4096;
    let tables_to_load = current_virtual.dir_offset() - start.dir_offset() + 1;
    for i in 0..(tables_to_load) {
        let address = VirtAddress(start.0 + 4 * 1024 * 1024 * i as u32);
        let table_addr : u32 = PTS_ADDR + 
            ((LOADED_PAGE_TABLES + i as u32) * TABLE_SIZE);

        println!("Mapping table at:\n\tdir: {}, frame_index: {}, offset : {}, \
            table_address : {:x}",
            address.dir_offset(),
            address.frame_index(), address.offset(),
            table_addr);

        (*PDT).pde[address.dir_offset()] = PageDirectoryEntry {
            address : table_addr, 
            avl : 0xf,
            ps : false,
            avl2 : false,
            accessed : false,
            cannot_cache : false,
            pwt : true,
            u_s : true,
            r_w : true,
            p : true,
        }.into();
        /*
        println!("{:x}", PageDirectoryEntry::from(
                PDT.pde[address.dir_offset()]
                )
            );
            */

        //We are storing page tables as they come.
        //Contiguos loaded pages do not correspond to contiguos virtual
        //memory addresses.
    }
    LOADED_PAGE_TABLES += tables_to_load as u32;
    println!("Currently loaded page_tables: {}", LOADED_PAGE_TABLES);
}



