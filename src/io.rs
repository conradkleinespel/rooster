use ansi_term::Color::{Green, Red, Yellow};
use ansi_term::Style;
use rpassword::{read_password_from_tty, read_password_with_reader};
use safe_string::SafeString;
use std::io::Write;
use std::io::{BufRead, Result as IoResult};

pub struct ReaderManager<'a, R: BufRead> {
    inner: &'a mut Box<R>,
    test: bool,
}

impl<'a, R: BufRead> ReaderManager<'a, R> {
    pub fn new(inner: &'a mut Box<R>, test: bool) -> ReaderManager<'a, R> {
        ReaderManager { inner, test }
    }

    pub fn read_line(&mut self) -> IoResult<String> {
        let mut s = String::new();
        self.inner.read_line(&mut s)?;
        Ok(s)
    }

    pub fn read_password(&mut self) -> IoResult<SafeString> {
        Ok(SafeString::new(if self.test {
            read_password_with_reader(Some(self.inner))?
        } else {
            read_password_from_tty(None)?
        }))
    }
}

pub struct WriterHandle<'a, T: Write + ?Sized> {
    writer: &'a mut Box<T>,
}

impl<'a, T: Write + ?Sized> WriterHandle<'a, T> {
    pub fn newline(&'a mut self) {
        self.writer.write_all("\n".as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn prompt(&'a mut self, s: &str) {
        self.writer.write_all(s.as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn raw(&'a mut self, s: &str) {
        self.writer.write_all(format!("{}", s).as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn title(&'a mut self, s: &str) {
        self.writer
            .write_all(format!("{}\n", Style::new().underline().bold().paint(s)).as_bytes())
            .unwrap();
        self.writer.flush().unwrap();
    }

    pub fn info(&'a mut self, s: &str) {
        self.writer
            .write_all(format!("{}\n", s).as_bytes())
            .unwrap();
        self.writer.flush().unwrap();
    }

    pub fn warning(&'a mut self, s: &str) {
        self.writer
            .write_all(format!("{}\n", Yellow.paint(s)).as_bytes())
            .unwrap();
        self.writer.flush().unwrap();
    }

    pub fn error(&'a mut self, s: &str) {
        self.writer
            .write_all(format!("{}\n", Red.paint(s)).as_bytes())
            .unwrap();
        self.writer.flush().unwrap();
    }

    pub fn success(&'a mut self, s: &str) {
        self.writer
            .write_all(format!("{}\n", Green.paint(s)).as_bytes())
            .unwrap();
        self.writer.flush().unwrap();
    }
}

pub struct WriterManager<
    'a,
    ErrorWriter: Write + ?Sized,
    OutputWriter: Write + ?Sized,
    InstructionWriter: Write + ?Sized,
> {
    error_writer: &'a mut Box<ErrorWriter>,
    output_writer: &'a mut Box<OutputWriter>,
    instruction_writer: &'a mut Box<InstructionWriter>,
}

impl<
        'a,
        ErrorWriter: Write + ?Sized,
        OutputWriter: Write + ?Sized,
        InstructionWriter: Write + ?Sized,
    > WriterManager<'a, ErrorWriter, OutputWriter, InstructionWriter>
{
    pub fn new(
        error_writer: &'a mut Box<ErrorWriter>,
        output_writer: &'a mut Box<OutputWriter>,
        instruction_writer: &'a mut Box<InstructionWriter>,
    ) -> WriterManager<'a, ErrorWriter, OutputWriter, InstructionWriter> {
        WriterManager {
            error_writer,
            output_writer,
            instruction_writer,
        }
    }
    pub fn error<'b>(&'b mut self) -> WriterHandle<'b, ErrorWriter> {
        WriterHandle {
            writer: self.error_writer,
        }
    }

    pub fn output<'b>(&'b mut self) -> WriterHandle<'b, OutputWriter> {
        WriterHandle {
            writer: self.output_writer,
        }
    }

    pub fn instruction<'b>(&'b mut self) -> WriterHandle<'b, InstructionWriter> {
        WriterHandle {
            writer: self.instruction_writer,
        }
    }
}
