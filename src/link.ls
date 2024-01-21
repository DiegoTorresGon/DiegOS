ENTRY(_start)
EXTERN(_start)

SECTIONS {
    . = 0x1000;

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
