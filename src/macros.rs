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
macro_rules! print_stderr(
    ($($arg:tt)*) => (
        match write!(&mut ::std::io::stderr(), $($arg)*) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

#[macro_export]
macro_rules! print_now(
    ($($arg:tt)*) => (
        match write!(&mut ::std::io::stdout(), $($arg)*) {
            Ok(_) => {
                ::std::io::stdout().flush().unwrap();
            },
            Err(x) => panic!("{}", x),
        }
    )
);

#[macro_export]
macro_rules! fgcolor(
    ($c:expr, $($args:tt)*) => (
        format!("{}{}\x1b[39m", $c.to_color_code(), format!($($args)*))
    )
);

#[macro_export]
macro_rules! errln(
    ($($args:tt)*) => (
        println_stderr!("{}", fgcolor!(Color::Red, $($args)*))
    )
);

#[macro_export]
macro_rules! okln(
    ($($args:tt)*) => (
        println_stderr!("{}", fgcolor!(Color::Green, $($args)*))
    )
);
