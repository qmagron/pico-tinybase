/********************************
 *             Base             *
 ********************************/

.set ROM_BASE, 0x00000000
.set XIP_BASE, 0x10000000
.set XIP_SSI_BASE, 0x18000000
.set PADS_QSPI_BASE, 0x40020000
.set PPB_BASE, 0xe0000000


/********************************
 *              ROM             *
 ********************************/

.set func_table, 0x14
.set table_lookup, 0x18
.set flash_enter_cmd_xip, ('X' << 8) + 'C'


/********************************
 *            XIP_SSI           *
 ********************************/

.set CTRLR0, 0x00
.set CTRLR1, 0x04
.set SSIENR, 0x08
.set BAUDR, 0x14
.set SR, 0x28
.set DR0, 0x60
.set RX_SAMPLE_DLY, 0xf0
.set SPI_CTRLR0, 0xf4

// XIP_SSI_CTRL0
.set SPI_FRF, 21
.set DFS_32, 16
.set TMOD, 8

// XIP_SSI_SR
.set SR_TFE, 0b0000100
.set SR_BUSY, 0b0000001

// XIP_SSI_SPI_CTRL0
.set XIP_CMD, 24
.set WAIT_CYCLES, 11
.set INST_L, 8
.set ADDR_L, 2
.set TRANS_TYPE, 0


/********************************
 *           PADS_QSPI          *
 ********************************/

.set QSPI_SCLK, 0x04
.set QSPI_SD0, 0x08
.set QSPI_SD1, 0x10
.set QSPI_SD2, 0x14
.set QSPI_SD3, 0x18

.set DRIVE, 4
.set SCHMITT, 1
.set SLEWFAST, 0


/********************************
 *          Cortex-M0+          *
 ********************************/

.set VTOR, 0xed08
