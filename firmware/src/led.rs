use crate::rp2040::*;


#[inline]
pub fn setup() {
    unsafe {
        // Enable GPIO
        RESET.write_volatile(RESET.read_volatile() & !IO_BANK0_RESET);

        // Set up LED GPIO
        GPIO25_CTRL.write_volatile(GPIO_FUNCTION_SIO);
        GPIO_OE_SET.write_volatile(LED_GPIO);
        GPIO_OUT_CLR.write_volatile(LED_GPIO);
    }
}


#[inline(always)]
pub fn toggle() {
    unsafe {
        GPIO_OUT_XOR.write_volatile(LED_GPIO);
    }
}
