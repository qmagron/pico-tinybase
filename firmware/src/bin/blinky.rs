#![no_std]
#![no_main]


use firmware::{entry, led};
use firmware::time::sleep;



entry!(main);

fn main() -> ! {
    led::setup();

    loop {
        led::toggle();
        sleep(1);
    }
}
