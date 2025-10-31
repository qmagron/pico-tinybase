use std::process::Command;


/** Run Makefile. */
fn make() -> bool {
    Command::new("make").status().unwrap().success()
}


fn main() -> Result<(), String> {
    match make() {
        true => Ok(()),
        false => Err(String::from("Makefile error")),
    }
}
