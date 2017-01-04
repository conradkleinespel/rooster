// Copyright 2014-2017 The Rooster Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ptr;
use super::libc;

#[allow(non_camel_case_types)]
pub type time_t = libc::c_uint;

mod internal {
    extern "C" {
        pub fn time(t: *mut super::time_t) -> super::time_t;
    }
}

pub fn time() -> time_t {
    let retrieved_time = unsafe { internal::time(ptr::null_mut()) };

    if retrieved_time == (!0 as u32) {
        panic!("Could not get time from system");
    }

    retrieved_time
}
