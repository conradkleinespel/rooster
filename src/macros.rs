#![macro_use]

#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::old_io::stdio::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

#[macro_export]
macro_rules! print_stderr(
    ($($arg:tt)*) => (
        match write!(&mut ::std::old_io::stdio::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
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
