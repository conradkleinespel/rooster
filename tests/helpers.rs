pub mod prelude {
    extern crate rooster;
    extern crate tempfile;

    pub use self::rooster::io::{ReaderManager, WriterManager};
    pub use self::rooster::main_with_args;
    pub fn sink() -> Box<Cursor<Vec<u8>>> {
        Box::new(Cursor::new(Vec::new()))
    }
    pub fn tempfile() -> PathBuf {
        self::tempfile::NamedTempFile::new()
            .unwrap()
            .path()
            .to_path_buf()
    }
    pub use std::io::Cursor;
    use std::path::PathBuf;
}

#[macro_export]
macro_rules! output {
    ( $x:expr, $y:expr, $z:expr ) => {
        &mut WriterManager::new($x, $y, $z)
    };
}

#[macro_export]
macro_rules! input {
    ( $x:expr ) => {
        &mut ReaderManager::new(&mut Box::new(Cursor::new($x.as_bytes().to_owned())), true)
    };
}
