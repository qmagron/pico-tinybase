#/usr/bin/env python3
#
# Add the required CRC32 checksum to the RP2040 second stage bootloader.

import sys


BITS = 32
MSB = 1 << (BITS - 1)
MASK = (1 << BITS) - 1

POLYNOMIAL = 0x04c11db7
INITIAL_VALUE = 0xffffffff
FINAL_XOR = 0x00000000


def rp2040_crc32(data: bytes):
    """Compute the CRC32 checksum for the RP2040 second stage bootloader."""
    checksum = INITIAL_VALUE

    for byte in data:
        b = 0x80

        while b != 0:
            bit = checksum & MSB

            checksum <<= 1

            if byte & b != 0:
                bit ^= MSB

            if bit != 0:
                checksum ^= POLYNOMIAL

            b >>= 1

    return (checksum ^ FINAL_XOR) & MASK


if __name__ == "__main__":
    with open(sys.argv[1], "rb+") as file:
        data = file.read()
        checksum = rp2040_crc32(data)
        file.write(bytearray.fromhex(f"{checksum:08x}")[::-1])
