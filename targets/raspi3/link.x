ENTRY(_start);

SECTIONS
{
    .text 0x80000 :
    {
        *(.text._start .text .text.*);
    }
}
