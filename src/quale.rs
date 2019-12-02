// Copyright (c) 2017 The rust-quale Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{env, ffi, fs, path};

use std::os::unix::fs::PermissionsExt;

pub fn which<S: AsRef<ffi::OsStr>>(name: S) -> Option<path::PathBuf> {
    let name: &ffi::OsStr = name.as_ref();

    let var = match env::var_os("PATH") {
        Some(var) => var,
        None => return None,
    };

    // Separate PATH value into paths
    let paths_iter = env::split_paths(&var);

    // Attempt to read each path as a directory
    let dirs_iter = paths_iter.filter_map(|path| fs::read_dir(path).ok());

    for dir in dirs_iter {
        let mut matches_iter = dir
            .filter_map(|file| file.ok())
            .filter(|file| file.file_name() == name)
            .filter(is_executable);
        if let Some(file) = matches_iter.next() {
            return Some(file.path());
        }
    }

    None
}

fn is_executable(file: &fs::DirEntry) -> bool {
    // Don't use `file.metadata()` directly since it doesn't follow symlinks.
    let file_metadata = match file.path().metadata() {
        Ok(metadata) => metadata,
        Err(..) => return false,
    };
    let file_path = match file.path().to_str().and_then(|p| ffi::CString::new(p).ok()) {
        Some(path) => path,
        None => return false,
    };
    let is_executable_by_user =
        unsafe { libc::access(file_path.into_raw(), libc::X_OK) == libc::EXIT_SUCCESS };
    static EXECUTABLE_FLAGS: u32 = (libc::S_IEXEC | libc::S_IXGRP | libc::S_IXOTH) as u32;
    let has_executable_flag = file_metadata.permissions().mode() & EXECUTABLE_FLAGS != 0;
    is_executable_by_user && has_executable_flag && file_metadata.is_file()
}

#[cfg(test)]
mod tests {
    use super::which;
    use std::path;

    /// FIXME: this is not a good test since it relies on PATH and the
    ///        filesystem being in a certain state.
    #[test]
    fn test_sh() {
        let expected = path::PathBuf::from("/usr/bin/sh");
        let actual = which("sh");
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn test_none() {
        let actual = which("foofoofoobar");
        assert_eq!(None, actual);
    }
}
