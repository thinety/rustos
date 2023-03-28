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

    .symtab : { *(.symtab) }
    .strtab : { *(.strtab) }
    .shstrtab : { *(.shstrtab) }

    /DISCARD/ :
    {
        *(.comment)
    }
}
