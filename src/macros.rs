#![macro_use]
#[allow(unused_imports)]
use std::io::Write;
use ansi_term::Color::{Red, Green};
use ansi_term::Style;

#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        if let Err(x) = writeln!(&mut ::std::io::stderr(), $($arg)*) {
            panic!("Unable to write to stderr: {}", x);
        }
    )
);

pub fn show_error(s: &str) {
    println_stderr!(
        "{}",
        Red.paint(s)
    )
}

pub fn show_ok(s: &str) {
    println!(
        "{}",
        Green.paint(s)
    )
}

pub fn show_title_1(s: &str) {
    println!(
        "{}",
        Style::new().underline().bold().paint(s)
    )
}

#[macro_export]
macro_rules! print_stderr(
    ($($arg:tt)*) => (
        write!(::std::io::stderr(), $($arg)*).and_then(|_| ::std::io::stderr().flush()).unwrap()
    )
);
