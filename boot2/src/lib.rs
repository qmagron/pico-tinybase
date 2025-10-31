#![no_std]

pub static BOOT2_03H: [u8; 256] = *include_bytes!("../bin/xip_03h.bin");
// pub static BOOT2_0BH: [u8; 256] = *include_bytes!("../bin/xip_0Bh.bin");
pub static BOOT2_3BH: [u8; 256] = *include_bytes!("../bin/xip_3Bh.bin");
pub static BOOT2_6BH: [u8; 256] = *include_bytes!("../bin/xip_6Bh.bin");
pub static BOOT2_BBH: [u8; 256] = *include_bytes!("../bin/xip_BBh.bin");
pub static BOOT2_EBH: [u8; 256] = *include_bytes!("../bin/xip_EBh.bin");
