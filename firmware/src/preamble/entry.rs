/// Define a function as the program entry point.
///
/// Example:
///
/// ```rs
/// entry!(main);
///
/// fn main() -> ! { /* ... */ }
/// ```
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        use core::arch::asm;

        #[used]
        #[unsafe(link_section = ".vector_table.reset_vector")]
        static __RESET_VECTOR: unsafe extern "C" fn() -> ! = __main;

        pub unsafe extern "C" fn __main() -> ! {
            // Force the type of $path
            let f: fn() -> ! = $path;

            unsafe {
                // Initialize SRAM
                asm!(include_str!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/src/preamble/memory.S")
                ));
            }

            f();
        }
    };
}
