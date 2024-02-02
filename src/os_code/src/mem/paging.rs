//For referece about this implementation of paging: https://wiki.osdev.org/Paging#32-bit_Paging_.28Protected_Mode.29

// We want to set up 32-bit paging with 4KByte pages.
// According to intel IA-32 architecture developer manual, we must set up like this:

// CR0.PG = 1 Enables paging. Must have protection enable CR0.PE = 1.
// CR4.PAE = 0 uses 32-bit paging mode,
// IA32_EFER.LME = 0. This is ensured by the processor when CR0.PG = 1 && CR4.PAE = 0.
// CR4.PSE = 0 to use 4kiB pages.

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PageDirectoryTable {
    pub pde : [PDEMemRep ; 1024]
}

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PageTable {
    pub page_frames : [PageFrameMemRep ; 1024]
}

#[derive(Clone, Copy)]
#[repr(align(4096))]
pub struct PageDirectoryEntry {
    address : u32, //Physical address of page table referenced by this entry, 31:12
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

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PDEMemRep(u32);

#[derive(Clone, Copy)]
#[repr(align(4096))]
pub struct PageFrame {
    address : u32, //Physical address of 4KByte page referenced by this entry. Bits 31:12
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

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PageFrameMemRep(u32);

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

    pub fn to_repr(self) -> PDEMemRep {
        PDEMemRep(
            (self.address << 12) | ((self.avl as u32) << 8) |
            ((self.ps as u32) << 7) | ((self.avl2 as u32) << 6) |
            ((self.accessed as u32) << 5) | ((self.cannot_cache as u32) << 4) |
            ((self.pwt as u32) << 3) | ((self.u_s as u32) << 2) |
            ((self.r_w as u32) << 1) | (self.p as u32)
        )
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

    pub fn to_repr(self) -> PageFrameMemRep {
        PageFrameMemRep(
            (self.address << 12) | ((self.avl as u32) << 9) |
            ((self.g as u32) << 8) | ((self.pat as u32) << 7) | 
            ((self.dirty as u32) << 6) | ((self.accessed as u32) << 5) | 
            ((self.pcd as u32) << 4) | ((self.pwt as u32) << 3) | 
            ((self.u_s as u32) << 2) | ((self.r_w as u32) << 1) | 
            (self.p as u32)
        )
    }

}

//I
pub fn init() {
    let mut directory = PageDirectoryTable{
        pde : [PageDirectoryEntry::null().to_repr(); 1024]
    };

    let mut page_table1 = PageTable {
        page_frames : [PageFrame::null().to_repr(); 1024]
    };

    for i in 0..250 { //We want to load 250 pages, 
        page_table1.page_frames[i] = PageFrame {
            address : (i * 4096) as u32,
            avl : 0,
            g : true,
            pat : false,
            dirty : false,
            accessed : false,
            pcd : false,
            pwt : true,
            u_s : false,
            r_w : false,
            p : true,
        }.to_repr();
    }

    directory.pde[0] = PageDirectoryEntry {
        address : (&page_table1 as *const _) as u32,
        avl : 0,
        ps : false,
        avl2 : false,
        accessed : false,
        cannot_cache : false,
        pwt : true,
        u_s : false,
        r_w : true,
        p : true,
    }.to_repr()

}

//next step is to map a memory region after 1MiB to use as general memory and for the heap.



