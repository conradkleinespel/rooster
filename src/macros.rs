#![macro_use]
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
macro_rules! println_err(
    ($($args:tt)*) => (
        println_stderr!(
            "{}",
            format!(
                "{}{}{}",
                ::color::Color::Red.to_color_code(),
                format!($($args)*),
                ::color::Color::Reset.to_color_code()
            )
        )
    )
);

#[macro_export]
macro_rules! println_ok(
    ($($args:tt)*) => (
        println!(
            "{}",
            format!(
                "{}{}{}",
                ::color::Color::Green.to_color_code(),
                format!($($args)*),
                ::color::Color::Reset.to_color_code()
            )
        )
    )
);

#[macro_export]
macro_rules! println_title(
    ($($args:tt)*) => (
        println!(
            "{}",
            format!(
                "{}{}{}",
                ::color::Color::Cyan.to_color_code(),
                format!($($args)*),
                ::color::Color::Reset.to_color_code()
            )
        )
    )
);

#[macro_export]
macro_rules! print_stderr(
    ($($arg:tt)*) => (
        write!(::std::io::stderr(), $($arg)*).and_then(|_| ::std::io::stderr().flush()).unwrap()
    )
);
