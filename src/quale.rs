// Copyright (c) 2017 The rust-quale Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Original code was copied from this crate:
// https://crates.io/crates/quale
// It has been modified here to support Windows.

use std::{env, ffi, path, fs};

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

#[cfg(windows)]
mod windows {
    use std::fs;
    use std::ptr::null_mut;
    use windows::core::PCWSTR;
    use windows::Win32::System::WindowsProgramming::{SCS_32BIT_BINARY, SCS_64BIT_BINARY, SCS_DOS_BINARY, SCS_OS216_BINARY, SCS_PIF_BINARY, SCS_POSIX_BINARY, SCS_WOW_BINARY};
    use windows::Win32::Storage::FileSystem::GetBinaryTypeW;
    use std::os::windows::ffi::OsStrExt;

    pub fn is_executable(file: &fs::DirEntry) -> bool {
        let windows_str = file.path().as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();
        let windows_string_ptr = PCWSTR(windows_str.as_ptr());
        let mut binary_type: u32 = 0;
        let binary_type_ptr: *mut u32 = &mut binary_type;

        let binary_type_res = unsafe {
            // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew
            GetBinaryTypeW(windows_string_ptr, binary_type_ptr)
        };

        if let Err(_) = binary_type_res {
            return false;
        }
        if binary_type_ptr == null_mut() {
            return false;
        }

        match binary_type {
            SCS_32BIT_BINARY | SCS_64BIT_BINARY | SCS_DOS_BINARY | SCS_OS216_BINARY | SCS_PIF_BINARY | SCS_POSIX_BINARY | SCS_WOW_BINARY => true,
            _ => false
        }
    }
}

#[cfg(unix)]
mod unix {
    use std::os::unix::fs::PermissionsExt;
    use std::fs;
    use std::ffi;

    pub fn is_executable(file: &fs::DirEntry) -> bool {
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
}

#[cfg(unix)]
use unix::is_executable;
#[cfg(windows)]
use windows::is_executable;

#[cfg(test)]
mod tests {
    use super::which;
    use std::path;

    #[cfg(unix)]
    #[test]
    fn test_sh() {
        let expected = path::PathBuf::from("/usr/bin/env");
        let actual = which("env");
        assert_eq!(Some(expected), actual);
    }

    #[cfg(windows)]
    #[test]
    fn test_sh() {
        let expected = path::PathBuf::from("C:\\Windows\\system32\\where.exe");
        let actual = which("where.exe");
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn test_none() {
        let actual = which("foofoofoobar");
        assert_eq!(None, actual);
    }
}
