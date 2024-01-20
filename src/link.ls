ENTRY(_start)

SECTIONS {
    . = 0x1000;

    .text :
    {
        *(.text._start)
        *(.text*)
    }

    .rodata* : {
        *(.rodata*)
    }
}
