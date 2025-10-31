// ----------------------------------------------------------------------------
// Second stage boot code
// Copyright (c) 2019-2021 Raspberry Pi (Trading) Ltd.
// Copyright (c) 2025 Quentin Magron
// SPDX-License-Identifier: BSD-3-Clause
//
// Building:    * This code must be position-independent, and use stack only
//              * The code will be padded to a size of 256 bytes, including a
//                4-byte checksum. Therefore code size cannot exceed 252 bytes.
// ----------------------------------------------------------------------------


.include "rp2040.S"

.cpu cortex-m0plus
.syntax unified
.thumb


/********************************
 *          Parameters          *
 ********************************/

.set SCKDV, 2

// XIP_SSI_CTRL0
.set SPI_FRF_V, 0x2       // STD DUAL [QUAD]
.set DFS_32_V, 31         // Dataframe size = n+1
.set TMOD_V, 0x3          // TX_AND_RX TX_ONLY RX_ONLY [EEPROM_READ]

// XIP_SSI_SPI_CTRL0
.set XIP_CMD_V, 0xeb      // Fast Read Quad I/O
.set WAIT_CYCLES_V, 4     // Dummy cycles
.set INST_L_V, 2          // NONE 4b [8b] 16b
.set ADDR_L_V, 8          // Address+Mode length = n*4
.set TRANS_TYPE_V, 0x1    // 1C1A [1C2A] 2C2A

.set CTRLR0_V, (SPI_FRF_V << SPI_FRF) | (DFS_32_V << DFS_32) | (TMOD_V << TMOD)
.set SPI_CTRLR0_V0, (XIP_CMD_V << XIP_CMD) | (WAIT_CYCLES_V << WAIT_CYCLES) | (INST_L_V << INST_L) | (ADDR_L_V << ADDR_L) | (TRANS_TYPE_V << TRANS_TYPE)
.set SPI_CTRLR0_V1, (   0xa0   << XIP_CMD) | (WAIT_CYCLES_V << WAIT_CYCLES) | (    0    << INST_L) | (ADDR_L_V << ADDR_L) | (TRANS_TYPE_V << TRANS_TYPE)


/********************************
 *          Bootloader          *
 ********************************/

.section .text

boot2:
    push {lr}

    ldr r3, =PADS_QSPI_BASE

    // Higher drive strength & Faster slew rate
    movs r0, #(2 << DRIVE | SLEWFAST)
    str r0, [r3, #QSPI_SCLK]

    // Disable Schmitt triggers
    ldr r0, [r3, #QSPI_SD0]
    movs r1, #SCHMITT
    bics r0, r1
    str r0, [r3, #QSPI_SD0]
    str r0, [r3, #QSPI_SD1]
    str r0, [r3, #QSPI_SD2]
    str r0, [r3, #QSPI_SD3]

    ldr r3, =XIP_SSI_BASE

    // Disable SSI
    movs r1, #0
    str r1, [r3, #SSIENR]

    // Set baud rate
    movs r1, #SCKDV
    str r1, [r3, #BAUDR]

    // Prevent race conditions
    movs r1, #1
    movs r2, #RX_SAMPLE_DLY
    str r1, [r3, r2]

    // Set up SSI & SPI
    ldr r1, =CTRLR0_V
    str r1, [r3, #CTRLR0]

    movs r1, #0x0
    str r1, [r3, #CTRLR1]

    ldr r1, =SPI_CTRLR0_V0
    movs r2, #SPI_CTRLR0
    str r1, [r3, r2]

    // Re-enable SSI
    movs r1, #1
    str r1, [r3, #SSIENR]

    // Manual SSI read on 0x000000 address + 0xa0 mode
    movs r1, #XIP_CMD_V
    str r1, [r3, #DR0]
    movs r1, #0xa0
    str r1, [r3, #DR0]
    bl wait_ssi_ready

    // Disable SSI
    movs r1, #0
    str r1, [r3, #SSIENR]

    // Set up SSI & SPI
    ldr r1, =SPI_CTRLR0_V1
    ldr r2, =(XIP_SSI_BASE + SPI_CTRLR0)
    str r1, [r2]

    // Re-enable SSI
    movs r1, #1
    str r1, [r3, #SSIENR]

.include "entry.S"

.include "ssi.S"

literals:
.ltorg

.end
