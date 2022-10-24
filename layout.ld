SECTIONS
{
    . = 0x80000;

    .text :
    {
        KEEP(*(.text.init))
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .shstrtab :
    {
        *(.shstrtab)
    }

    /DISCARD/ :
    {
        *(.*)
    }
}
