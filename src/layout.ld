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

    .bss :
    {
        *(.bss .bss.*)
    }

    .symtab : { *(.symtab) }
    .strtab : { *(.strtab) }
    .shstrtab : { *(.shstrtab) }

    /DISCARD/ :
    {
        *(.comment)
    }
}
