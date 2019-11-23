use atty::Stream;

// https://crates.io/crates/atty
// https://rosettacode.org/wiki/Check_output_device_is_a_terminal
fn main() {  
    if atty::is(Stream::Stdout) {
        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }

    let istty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
    if istty {
        println!("stdout is tty");
    } else {
        println!("stdout is not tty");
    }
}
