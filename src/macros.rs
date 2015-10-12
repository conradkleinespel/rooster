#![macro_use]
#[allow(unused_imports)]
use std::io::Write;

#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)*) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

#[macro_export]
macro_rules! println_err(
    ($($args:tt)*) => (
        println_stderr!("{}", format!("{}{}\x1b[39m", ::color::Color::Red.to_color_code(), format!($($args)*)))
    )
);

#[macro_export]
macro_rules! println_ok(
    ($($args:tt)*) => (
        println_stderr!("{}", format!("{}{}\x1b[39m", ::color::Color::Green.to_color_code(), format!($($args)*)))
    )
);
