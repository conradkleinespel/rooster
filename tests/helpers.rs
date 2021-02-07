pub mod prelude {

    pub use rooster::main_with_args;
    pub use rooster::rclio::CursorInputOutput;
    pub fn tempfile() -> PathBuf {
        tempfile::NamedTempFile::new().unwrap().path().to_path_buf()
    }
    pub use std::io::Cursor;
    use std::path::PathBuf;
}
