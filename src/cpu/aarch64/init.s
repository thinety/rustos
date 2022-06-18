.section .text.init

_start:
    adr x0, _start
    mov sp, x0

    b   main

.size _start, . - _start
.type _start, function
.global _start
