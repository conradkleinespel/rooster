use crate::rutil::atty;
use crate::rutil::safe_string::SafeString;
use ansi_term::Color::{Green, Red, Yellow};
use ansi_term::Style as AnsiTermStyle;
use rpassword::{
    prompt_password, prompt_password_from_bufread, read_password, read_password_from_bufread,
};
use rprompt::{prompt_reply, prompt_reply_from_bufread, read_reply, read_reply_from_bufread};
use std::io::Result as IoResult;
use std::io::{Cursor, StderrLock, StdinLock, StdoutLock, Write};

pub enum OutputType {
    Standard,
    Error,
}

/// Struct that reads and writes data from the TTY, stdin and stdout
pub struct RegularInputOutput<'a> {
    stdin_lock: StdinLock<'a>,
    stdout_lock: StdoutLock<'a>,
    stderr_lock: StderrLock<'a>,
}

impl<'a> RegularInputOutput<'a> {
    pub fn new<'b>(
        stdin_lock: StdinLock<'b>,
        stdout_lock: StdoutLock<'b>,
        stderr_lock: StderrLock<'b>,
    ) -> RegularInputOutput<'b> {
        RegularInputOutput {
            stdin_lock,
            stdout_lock,
            stderr_lock,
        }
    }
}

/// Struct similar to `RegularInputOutput` but that reads and writes from a cursor, useful for tests
#[derive(Default)]
pub struct CursorInputOutput {
    pub stdin_cursor: Cursor<Vec<u8>>,
    pub ttyin_cursor: Cursor<Vec<u8>>,
    pub stdout_cursor: Cursor<Vec<u8>>,
    pub stderr_cursor: Cursor<Vec<u8>>,
    pub ttyout_cursor: Cursor<Vec<u8>>,
}

impl CursorInputOutput {
    pub fn new(stdin: &str, ttyin: &str) -> CursorInputOutput {
        CursorInputOutput {
            stdin_cursor: Cursor::new(stdin.as_bytes().to_owned()),
            ttyin_cursor: Cursor::new(ttyin.as_bytes().to_owned()),
            stdout_cursor: Cursor::new(Vec::new()),
            stderr_cursor: Cursor::new(Vec::new()),
            ttyout_cursor: Cursor::new(Vec::new()),
        }
    }
}

pub trait CliInputOutput {
    fn read_line(&mut self) -> IoResult<String>;
    fn prompt_line(&mut self, prompt: impl ToString) -> IoResult<String>;
    fn read_password(&mut self) -> IoResult<SafeString>;
    fn prompt_password(&mut self, prompt: impl ToString) -> IoResult<SafeString>;

    fn nl(&mut self, output_type: OutputType);
    fn write(&mut self, s: impl ToString, output_type: OutputType);
    fn writeln(&mut self, s: impl ToString, output_type: OutputType);

    fn title(&mut self, s: impl ToString, output_type: OutputType) {
        self.writeln(
            AnsiTermStyle::new()
                .underline()
                .bold()
                .paint(s.to_string())
                .to_string(),
            output_type,
        )
    }

    fn info(&mut self, s: impl ToString, output_type: OutputType) {
        self.writeln(
            AnsiTermStyle::new().paint(s.to_string()).to_string(),
            output_type,
        )
    }

    fn warning(&mut self, s: impl ToString, output_type: OutputType) {
        self.writeln(
            Yellow.normal().paint(s.to_string()).to_string(),
            output_type,
        )
    }

    fn error(&mut self, s: impl ToString, output_type: OutputType) {
        self.writeln(Red.normal().paint(s.to_string()).to_string(), output_type)
    }

    fn success(&mut self, s: impl ToString, output_type: OutputType) {
        self.writeln(Green.normal().paint(s.to_string()).to_string(), output_type)
    }
}

impl<'a> CliInputOutput for RegularInputOutput<'a> {
    fn read_line(&mut self) -> IoResult<String> {
        if !atty::is(atty::Stream::Stdin) {
            panic!("Need a TTY to read password");
        }

        read_reply()
    }

    fn prompt_line(&mut self, prompt: impl ToString) -> IoResult<String> {
        if !atty::is(atty::Stream::Stdin) || !atty::is(atty::Stream::Stdout) {
            panic!("Need a TTY to read password");
        }

        prompt_reply(prompt)
    }

    fn read_password(&mut self) -> IoResult<SafeString> {
        if !atty::is(atty::Stream::Stdin) {
            panic!("Need a TTY to read password");
        }

        Ok(SafeString::from_string(read_password()?))
    }

    fn prompt_password(&mut self, prompt: impl ToString) -> IoResult<SafeString> {
        if !atty::is(atty::Stream::Stdin) || !atty::is(atty::Stream::Stdout) {
            panic!("Need a TTY to read password");
        }

        Ok(SafeString::from_string(prompt_password(prompt)?))
    }

    fn nl(&mut self, output_type: OutputType) {
        match output_type {
            OutputType::Standard => {
                self.stdout_lock.write_all("\n".as_bytes()).unwrap();
                self.stdout_lock.flush().unwrap();
            }
            OutputType::Error => {
                self.stderr_lock.write_all("\n".as_bytes()).unwrap();
                self.stderr_lock.flush().unwrap();
            }
        }
    }

    fn write(&mut self, s: impl ToString, output_type: OutputType) {
        match output_type {
            OutputType::Standard => {
                self.stdout_lock
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stdout_lock.flush().unwrap();
            }
            OutputType::Error => {
                self.stderr_lock
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stderr_lock.flush().unwrap();
            }
        }
    }

    fn writeln(&mut self, s: impl ToString, output_type: OutputType) {
        match output_type {
            OutputType::Standard => {
                self.stdout_lock
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stdout_lock.write_all("\n".as_bytes()).unwrap();
                self.stdout_lock.flush().unwrap();
            }
            OutputType::Error => {
                self.stderr_lock
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stderr_lock.write_all("\n".as_bytes()).unwrap();
                self.stderr_lock.flush().unwrap();
            }
        }
    }
}

impl CliInputOutput for CursorInputOutput {
    fn read_line(&mut self) -> IoResult<String> {
        read_reply_from_bufread(&mut self.ttyin_cursor)
    }

    fn prompt_line(&mut self, prompt: impl ToString) -> IoResult<String> {
        prompt_reply_from_bufread(&mut self.ttyin_cursor, &mut self.ttyout_cursor, prompt)
    }

    fn read_password(&mut self) -> IoResult<SafeString> {
        Ok(SafeString::from_string(read_password_from_bufread(
            &mut self.ttyin_cursor,
        )?))
    }

    fn prompt_password(&mut self, prompt: impl ToString) -> IoResult<SafeString> {
        Ok(SafeString::from_string(prompt_password_from_bufread(
            &mut self.ttyin_cursor,
            &mut self.ttyout_cursor,
            prompt,
        )?))
    }

    fn nl(&mut self, output_type: OutputType) {
        match output_type {
            OutputType::Standard => {
                self.stdout_cursor.write_all("\n".as_bytes()).unwrap();
                self.stdout_cursor.flush().unwrap();
            }
            OutputType::Error => {
                self.stderr_cursor.write_all("\n".as_bytes()).unwrap();
                self.stderr_cursor.flush().unwrap();
            }
        }
    }

    fn write(&mut self, s: impl ToString, output_type: OutputType) {
        match output_type {
            OutputType::Standard => {
                self.stdout_cursor
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stdout_cursor.flush().unwrap();
            }
            OutputType::Error => {
                self.stderr_cursor
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stderr_cursor.flush().unwrap();
            }
        }
    }

    fn writeln(&mut self, s: impl ToString, output_type: OutputType) {
        match output_type {
            OutputType::Standard => {
                self.stdout_cursor
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stdout_cursor
                    .write_all("\n".to_string().as_bytes())
                    .unwrap();
                self.stdout_cursor.flush().unwrap();
            }
            OutputType::Error => {
                self.stderr_cursor
                    .write_all(s.to_string().as_bytes())
                    .unwrap();
                self.stderr_cursor
                    .write_all("\n".to_string().as_bytes())
                    .unwrap();
                self.stderr_cursor.flush().unwrap();
            }
        }
    }
}
