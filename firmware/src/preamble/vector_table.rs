const N_EXCEPTIONS: usize = 15;
const N_INTERRUPTS: usize = 32;


type Handler = unsafe extern "C" fn() -> !;


unsafe extern "C" fn default_handler() -> ! { loop {} }


#[used]
#[unsafe(link_section = ".vector_table.exceptions")]
static EXCEPTIONS: [Option<Handler>; N_EXCEPTIONS-1] = [
    Some(default_handler),
    Some(default_handler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(default_handler),
    None,
    None,
    Some(default_handler),
    Some(default_handler),
];


#[used]
#[unsafe(link_section = ".vector_table.interrupts")]
static INTERRUPTS: [Handler; N_INTERRUPTS] = [default_handler; N_INTERRUPTS];
