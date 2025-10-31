.include "rp2040.S"

.cpu cortex-m0plus
.syntax unified
.thumb


.section .text

/** Wait for SSI task to complete */
wait_ssi_ready:
    push {lr}

    1:
        // Wait for TX FIFO empty
        ldr r1, [r3, #SR]
        movs r0, #SR_TFE
        tst r1, r0
        beq 1b

        // Wait for SSI inactive
        movs r0, #SR_BUSY
        tst r1, r0
        bne 1b

    pop {pc}
