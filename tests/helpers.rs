pub mod prelude {

    pub use rooster::CursorInputOutput;
    pub use rooster::main_with_args;
    pub fn tempfile() -> PathBuf {
        tempfile::NamedTempFile::new().unwrap().path().to_path_buf()
    }
    use std::path::PathBuf;
}
