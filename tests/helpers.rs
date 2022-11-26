pub mod prelude {

    pub use rclio::CursorInputOutput;
    pub use rooster::main_with_args;
    pub fn tempfile() -> PathBuf {
        tempfile::NamedTempFile::new().unwrap().path().to_path_buf()
    }
    pub use std::io::Cursor;
    use std::path::PathBuf;
}
