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
macro_rules! fgcolor(
    ($c:expr, $($args:tt)*) => (
        format!("{}{}\x1b[39m", $c.to_color_code(), format!($($args)*))
    )
);
