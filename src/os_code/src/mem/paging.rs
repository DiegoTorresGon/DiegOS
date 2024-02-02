//For referece about this implementation of paging: https://wiki.osdev.org/Paging#32-bit_Paging_.28Protected_Mode.29

// We want to set up 32-bit paging with 4KByte pages.
// According to intel IA-32 architecture developer manual, we must set up like this:

// CR0.PG = 1 Enables paging. Must have protection enable CR0.PE = 1.
// CR4.PAE = 0 uses 32-bit paging mode,
// IA32_EFER.LME = 0. This is ensured by the processor when CR0.PG = 1 && CR4.PAE = 0.
// CR4.PSE = 0 to use 4kiB pages.

pub struct PageDirectoryTable {
    pde : [PageTable: 1024]
}

pub struct PageTable {
    address : u32, //Physical address of page table referenced by this entry, 31:12
    avl : u8,  //This 3 bit field can be used freely by the Kernel. Bit 11:8
    ps : bool,  //must be 0 for 4kiB pages Bit 7
    avl2 : bool, //Can be used freely. Bit 6
    accesed : bool, //Indicates if page table has been used. Bit 5
    can_cache : bool, //If set, page will not be chached. Bit 4
    pwt : bool, //If set, write-through caching enabled, else write-back enabled. Bit 3
    u_s : bool, //If set to 0, user mode access is not enabled. Bit 2
    r_w : bool, //If set, writes are enabled. Bit 1
    p : bool, //Present. Bit 0

    pte : [PageFrame : 1024]
}

pub struct PageFrame {
    address : u32, //Physical address of 4KByte page referenced by this entry. Bits 31:12
    avl : u8, //Ignored. Bits 11:9
    g : bool, //Indicates if translation chached should be global. Bit 8
    pat : bool, //Should be set only if processor supports PAT. Bit 7
    dirty : bool, //Indicates if page has been written to. Bit 6
    accesed, bool, //Indicates if it has been accessed. Bit 5
    pcd : bool, //Page level chache disable. Bit 4
    pwt : bool, //Same as in PageTable. Bit 3
    u_s : bool, //Sames as in PageTable. Bit 2
    r_w : bool, //If set, writes are enabled. Bit 1
    p : bool, //Present. Bit 0
}

pub fn init() {

}



