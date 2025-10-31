use crate::rp2040::*;


#[inline]
pub fn set(rvr: u32) {
    unsafe {
        SYST_RVR.write_volatile(rvr);
        SYST_CSR.write_volatile(SYST_CSR.read_volatile() | SYST_CSR_ENABLE | SYST_CSR_CLKSOURCE);
    }
}


#[inline]
pub fn unset() {
    unsafe {
        SYST_CSR.write_volatile(SYST_CSR.read_volatile() & !(SYST_CSR_ENABLE | SYST_CSR_CLKSOURCE));
    }
}
