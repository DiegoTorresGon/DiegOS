ENTRY(_start)
EXTERN(_start)

MEMORY {
	ram0 (rwx) : ORIGIN = 0x9000, LENGTH = 492032
	ram1 (rwx) : ORIGIN = 0x100000, LENGTH = 14680063
}

SECTIONS {
    . = 0x9000;
	
    .text :
    {
        *(.text._start)
        *(.text*)
    }

	.page_directory :
	{
		*(.page_directory)
	}	

    .rodata : {
        *(.rodata*)
    }

	.data : {
		*(.data)
	}

	/DISCARD/ :
	{
		*(.comment)
	}
}
