const FREQ: u32 = 6;  // MHz
const CPU: u32 = FREQ / 4;  // Cycles / µSec
const USEC: u32 = 1_000_000;


/// Busy sleep for an *approximative* time (in seconds).
///
/// Actual time depends on core frequency.
#[inline(always)]
pub fn sleep(sec: u32) {
    usleep(sec * USEC);
}


/// Busy sleep for an *approximative* time (in microseconds).
///
/// Actual time depends on core frequency.
#[inline(always)]
pub fn usleep(usec: u32) {
    unsafe {
        core::arch::asm!(
            "movs {c}, {n}",
            "1:",
            "nop",                   //   1 cycle
            "subs {c}, 1",           // + 1 cycle
            "bne 1b",                // + taken ? 2 cycles : 1 cycle
            n = in(reg) CPU * usec,  // = ~4 cycles / rep
            c = out(reg) _,
        );
    }
}
