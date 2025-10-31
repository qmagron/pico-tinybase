This repository contains a tiny firmware for the Raspberry Pi Pico, with **absolutely zero dependencies**.

If you ever wanted to learn how the SDK works, but you didn't have the time or simply were too lazy to read thousands of lines of code across far too many files and dependencies, then this tiny firmware is exactly what you want.

**DISCLAIMER:** some features may not work properly because the system is not fully initialized.


## Dependencies

> Wait, you just said there are "absolutely zero" dependencies.

Although there are no code dependencies, we still need some tools to build the firmware.

1. [Rust](https://rust-lang.org/): to compile the firmware
2. [arm-none-eabi-binutils](https://www.gnu.org/software/binutils/): to assemble the bootloader
2. [Python](https://www.python.org/): to compute the bootloader checksum
3. [probe-rs](https://probe.rs/): to flash firmware on the board


## Build

First, you need to add the `thumbv6m-none-eabi` target:

```sh
rustup target add thumbv6m-none-eabi
```

Then, you can compile the binary you want:

```rs
cargo build --bin blinky
```

Or you can flash it on the board with [`probe-rs`](https://probe.rs/):

```rs
cargo flash --chip rp2040 --bin blinky
```
