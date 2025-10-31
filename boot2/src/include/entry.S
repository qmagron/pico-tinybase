.include "rp2040.S"

.cpu cortex-m0plus
.syntax unified
.thumb


.section .text

check_return:
    pop {r0}
    cmp r0, #0
    beq setup_vector_table
    bx r0

setup_vector_table:
    ldr r0, =(XIP_BASE + 0x100)
    ldr r1, =(PPB_BASE + VTOR)
    str r0, [r1]
    ldmia r0, {r0, r1}
    msr msp, r0
    bx r1
