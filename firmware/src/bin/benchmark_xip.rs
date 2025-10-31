#![no_std]
#![no_main]


use firmware::{entry, led};
use firmware::rp2040::*;
use firmware::systick;
use firmware::time::sleep;
use firmware::time::usleep;



const RVR: u32 = 0xffffff;



/** Returns the number of cycles for an uncached read in XIP memory. */
fn measure_read_cycles() -> u32 {
    systick::set(RVR);

    let n: u32;
    unsafe {
        core::arch::asm!(
            "movs {c}, 2",

            "1:",
            // 1. Start SysTick timer
            // 2. Uncached XIP memory access
            // 3. Read SysTick value
            "str {c}, [{cvr}]",
            "ldr {}, [{addr}]",
            "ldr {n}, [{cvr}]",

            // Repeat this 2 times:
            //   1. To load past instructions into cache
            //   2. To measure latency
            "subs {c}, 1",
            "bne 1b",
            out(reg) _,
            cvr = in(reg) SYST_CVR,
            addr = in(reg) XIP_NOCACHE_NOALLOC_BASE,
            c = out(reg) _,
            n = out(reg) n,
        );
    }

    systick::unset();

    RVR - n
}



entry!(main);

fn main() -> ! {
    led::setup();

    let n = measure_read_cycles();

    sleep(1);

    // Transmit binary #cycles with LED
    // Would probably be better with UART, but would require more set up
    for b in (0..24).rev() {
        // Skip leading zeroes
        if n >> b == 0 {
            continue;
        }

        let bit = (n >> b) & 1;

        // Blink short for 0 and long for 1
        led::toggle();
        usleep(if bit == 0 { 200_000 } else { 1_000_000 });
        led::toggle();

        sleep(1);
    }

    loop {}
}
