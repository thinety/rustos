.section .text.init

start:
    adr x0, start
    mov sp, x0

    b   main

.size start, . - start
.type start, function
.global start
