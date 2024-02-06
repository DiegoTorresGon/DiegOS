ENTRY(_start)
EXTERN(_start)

MEMORY {
	ram0 (rwx) : ORIGIN = 0x9000, LENGTH = 446464
	ram1 (rwx) : ORIGIN = 0x100000, LENGTH = 14680063
}

SECTIONS {
    . = 0x9000;
	
    .text :
    {
        *(.text._start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

	.data : {
		*(.data)
	}

	.page_directory :
	{
		*(.page_directory)
	}	

	/DISCARD/ :
	{
		*(.comment)
	}
}
