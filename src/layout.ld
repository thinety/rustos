ENTRY(start)

SECTIONS
{
    . = 0x80000;

    .text :
    {
        *(.text.init)
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
