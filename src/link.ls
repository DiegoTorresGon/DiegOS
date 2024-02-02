ENTRY(_start)
EXTERN(_start)

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

	/DISCARD/ :
	{
		*(.comment)
	}
}
