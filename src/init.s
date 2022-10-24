.section .text.init

start:
    adr x4, start
    mov sp, x4

    b   main

.size start, . - start
.type start, function
.global start
