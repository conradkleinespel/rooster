use crate::rutil::fix_new_line::fix_new_line;
use crate::rutil::print_tty::{print_tty, print_writer};
use std::io::{BufRead, BufReader, Write};

/// Reads user input from stdin
pub fn read_reply() -> std::io::Result<String> {
    read_reply_from_bufread(&mut get_tty_reader()?)
}

/// Reads user input from anything that implements BufRead
pub fn read_reply_from_bufread(reader: &mut impl BufRead) -> std::io::Result<String> {
    let mut reply = String::new();

    reader.read_line(&mut reply)?;

    fix_new_line(reply)
}

/// Displays a message on the TTY, then reads user input from stdin
pub fn prompt_reply(prompt: impl ToString) -> std::io::Result<String> {
    print_tty(prompt).and_then(|_| read_reply_from_bufread(&mut get_tty_reader()?))
}

/// Displays a message on the TTY, then reads user input from anything that implements BufRead
pub fn prompt_reply_from_bufread(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    prompt: impl ToString,
) -> std::io::Result<String> {
    print_writer(writer, prompt.to_string().as_str()).and_then(|_| read_reply_from_bufread(reader))
}

#[cfg(unix)]
fn get_tty_reader() -> std::io::Result<impl BufRead> {
    Ok(BufReader::new(
        std::fs::OpenOptions::new().read(true).open("/dev/tty")?,
    ))
}

#[cfg(windows)]
fn get_tty_reader() -> std::io::Result<impl BufRead> {
    use std::os::windows::io::FromRawHandle;
    use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE};

    let handle = unsafe {
        CreateFileA(
            b"CONIN$\x00".as_ptr() as *const i8,
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
    }

    Ok(BufReader::new(unsafe {
        std::fs::File::from_raw_handle(handle)
    }))
}
