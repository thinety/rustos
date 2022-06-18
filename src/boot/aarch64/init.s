.section .text.init

_start:
    b   main

.size _start, . - _start
.type _start, function
.global _start
