#![macro_use]
use ansi_term::Color::{Green, Red, Yellow};
use ansi_term::Style;
#[allow(unused_imports)]
use std::io::Write;

#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        if let Err(x) = writeln!(&mut ::std::io::stderr(), $($arg)*) {
            panic!("Unable to write to stderr: {}", x);
        }
    )
);

#[macro_export]
macro_rules! print_stderr(
    ($($arg:tt)*) => (
        write!(::std::io::stderr(), $($arg)*).and_then(|_| ::std::io::stderr().flush()).unwrap()
    )
);

pub fn show_error(s: &str) {
    println_stderr!("{}", Red.paint(s))
}

pub fn show_warning(s: &str) {
    println_stderr!("{}", Yellow.paint(s))
}

pub fn show_ok(s: &str) {
    println!("{}", Green.paint(s))
}

pub fn show_title_1(s: &str) {
    println!("{}", Style::new().underline().bold().paint(s))
}

pub fn write_to_stderr(s: &str) {
    print_stderr!("{}", s);
}
