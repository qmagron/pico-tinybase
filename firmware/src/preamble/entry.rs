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
        #[used]
        #[unsafe(link_section = ".vector_table.reset_vector")]
        static __RESET_VECTOR: unsafe extern "C" fn() -> ! = __main;

        pub unsafe extern "C" fn __main() -> ! {
            // Force the type of $path
            let f: fn() -> ! = $path;

            f();
        }
    };
}
