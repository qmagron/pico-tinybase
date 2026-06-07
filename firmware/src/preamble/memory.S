.cpu cortex-m0plus
.syntax unified
.thumb


.extern __sbss
.extern __ebss
.extern __sdata
.extern __edata
.extern __sidata


// Initialize .bss
    movs r2, #0
    ldr r0, =__sbss
    ldr r1, =__ebss
1:
    cmp r0, r1
    beq 2f
    strb r2, [r0]
    adds r0, #1
    b 1b

// Initialize .data
2:
    ldr r0, =__sdata
    ldr r1, =__edata
    ldr r2, =__sidata
1:
    cmp r0, r1
    beq 2f
    ldrb r3, [r2]
    strb r3, [r0]
    adds r0, #1
    adds r2, #1
    b 1b
2:
