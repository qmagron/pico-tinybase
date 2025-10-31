/********************************
 *             Base             *
 ********************************/

pub const XIP_NOCACHE_NOALLOC_BASE: u32 = 0x13000000;
pub const SIO_BASE: u32 = 0xd0000000;
pub const RESETS_BASE: u32 = 0x4000c000;
pub const IO_BANK0_BASE: u32 = 0x40014000;
pub const PPB_BASE: u32 = 0xe0000000;


/********************************
 *              SIO             *
 ********************************/

pub const GPIO_OUT_CLR: *mut u32 = (SIO_BASE + 0x018) as _;
pub const GPIO_OUT_XOR: *mut u32 = (SIO_BASE + 0x01c) as _;
pub const GPIO_OE_SET: *mut u32 = (SIO_BASE + 0x024) as _;

pub const LED_GPIO: u32 = 1 << 25;


/********************************
 *            RESETS            *
 ********************************/

pub const RESET: *mut u32 = (RESETS_BASE + 0x000) as _;

pub const IO_BANK0_RESET: u32 = 1 << 5;


/********************************
 *           IO_BANK0           *
 ********************************/

pub const GPIO25_CTRL: *mut u32 = (IO_BANK0_BASE + 0x0cc) as _;

pub const GPIO_FUNCTION_SIO: u32 = 5;


/********************************
 *          Cortex-M0+          *
 ********************************/

pub const SYST_CSR: *mut u32 = (PPB_BASE + 0xe010) as _;
pub const SYST_RVR: *mut u32 = (PPB_BASE + 0xe014) as _;
pub const SYST_CVR: *mut u32 = (PPB_BASE + 0xe018) as _;

pub const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
pub const SYST_CSR_ENABLE: u32 = 1 << 0;


/********************************
 *             Misc             *
 ********************************/

pub const NONE: u32 = 0;
pub const ALL: u32 = !NONE;
